//! Handling of 'pull' subcommand of 'image' command

use std::collections::HashMap;
use std::io;
use std::mem;
use std::sync::Arc;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use tokio::{io::BufReader, sync::Semaphore};

use crate::image::{
    oci::{
        digest::Digest,
        layout::OCIImageLayout,
        spec_v1::{Descriptor, Image as OCIImage, Index, Manifest},
    },
    transports,
    types::ImageSource,
};

/// API to subscribe to 'pull' subcommand
pub fn add_subcommand_pull() -> App<'static, 'static> {
    SubCommand::with_name("pull")
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .about("pull container image")
        .arg(
            Arg::with_name("name")
                .required(true)
                .help("Image name to pull")
                .index(1),
        )
        .arg(
            Arg::with_name("force")
                .help("Force pull the image.")
                .short("f")
                .long("force"),
        )
        .arg(
            Arg::with_name("no-clear")
                .help("Do not clear the local directory upon error. Useful during debugging.")
                .long("no-clear"),
        )
}

/// API to run 'pull' subcommand
pub async fn run_subcmd_pull(cmd: &ArgMatches<'_>) -> io::Result<()> {
    let image_name = cmd.value_of("name").unwrap();

    log::debug!("Image Name: {}", image_name);

    let force = cmd.is_present("force");
    let dont_clear = cmd.is_present("no-clear");

    let image_ref = transports::parse_image_name(image_name)?;
    let docker_ref = image_ref.docker_reference();

    if docker_ref.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid Image Name {}", image_name),
        ));
    }

    let name = docker_ref.as_ref().unwrap().name();
    let tag = docker_ref.as_ref().unwrap().tag();

    log::debug!("Name: {}, Tag: {}", name, tag);
    let mut img_layout = OCIImageLayout::new(&name, Some(&tag), None);

    if img_layout.fs_path_exists {
        if !force {
            let errstr = format!("Local FS path for the image with name: {}, tag: {} exists. Please specify `--force` to overwrite.", name, tag);
            log::error!("{}", errstr);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, errstr));
        } else {
            log::warn!("Local Image Layout exists, deleting...");
            img_layout.delete_fs_path().await?;
        }
    }

    img_layout.create_fs_path().await?;

    log::info!("Pulling the image: {}", image_name);

    let result = match perform_image_pull(&mut img_layout, image_name).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error : {}", e);
            if !dont_clear {
                img_layout.delete_fs_path().await?;
            }
            Err(e)
        }
    };

    result
}

async fn perform_image_pull(
    img_layout: &mut OCIImageLayout,
    image_name: &str,
) -> std::io::Result<()> {
    let image_ref = transports::parse_image_name(image_name)?;

    let mut img = image_ref.new_image()?;

    log::debug!("Getting Manifest for the Image.");
    let manifest = img.resolved_manifest().await?;

    log::debug!("Writing Manifest Blob.");
    let digest = Digest::from_bytes(&manifest.manifest);

    let mut reader = BufReader::new(&*manifest.manifest);
    img_layout.write_blob_file(&digest, &mut reader).await?;

    let mut annotations = HashMap::new();
    // FIXME : Not sure, Also, right now we 'know' tag is `Some`.
    let _ = annotations.insert(
        "org.opencontainers.image.ref.name".to_string(),
        img_layout.tag.as_ref().unwrap().clone(),
    );

    // Manifest written, now create index.json
    let manifest_descriptor = Descriptor {
        mediatype: Some(manifest.mime_type.to_string()),
        digest: digest,
        size: manifest.manifest.len() as i64,
        urls: None,
        platform: None,
        annotations: Some(annotations),
    };

    let index = Index {
        version: 2,
        manifests: vec![manifest_descriptor],
        annotations: None,
    };

    let _ = mem::replace(&mut img_layout.index, index);

    // Download and verify config
    log::debug!("Getting Image Config.");
    let manifest_obj: Manifest = serde_json::from_slice(&manifest.manifest)?;

    log::debug!("Saving Image Config.");
    let config = img.config_blob().await?;
    let mut reader = BufReader::new(&*config);
    img_layout
        .write_blob_file(&manifest_obj.config.digest, &mut reader)
        .await?;

    // Download and verify each of the layer blobs. If the blobs are gzipped
    // unzip the blobs (Don't unzip use unzip + reader) and then verify the signature
    // as mentioned in config rootfs. If fails - fail

    let image_obj: OCIImage = serde_json::from_slice(&config)?;

    log::debug!("Getting Image Layers!");
    let max_parallel_dloads = 3;
    let mut layer_handles = vec![];
    let semaphore = Arc::new(Semaphore::new(max_parallel_dloads));

    for (layer, unzipped_digest) in manifest_obj.layers.iter().zip(image_obj.rootfs.diff_ids) {
        let layer_digest = layer.digest.clone();
        let img_layout = img_layout.clone();
        let img_source = image_ref.new_image_source()?;

        let permit = semaphore.clone().acquire_owned().await;

        let handle = tokio::spawn(async move {
            do_download_image_layer(layer_digest, unzipped_digest, img_layout, img_source).await?;
            drop(permit);
            Ok::<(), std::io::Error>(())
        });
        layer_handles.push(handle);
    }

    for h in layer_handles {
        let _ = h.await?;
    }

    // We now have everything - Write this to disk layout.
    log::debug!("Writing 'index.json'.");
    img_layout.write_index_json().await?;

    log::debug!("Writing 'img-layout'.");
    img_layout.write_image_layout().await?;

    log::info!("Image downloaded and saved successfully!");
    Ok(())
}

async fn do_download_image_layer<'a>(
    layer_digest: Digest,
    unzipped_digest: Digest,
    img_layout: OCIImageLayout,
    //img: Box<dyn Image + Send + Sync>,
    img_source: Box<dyn ImageSource + Send + Sync>,
) -> io::Result<()> {
    log::info!("Getting Image Layer: {}", layer_digest);

    // let img_source = img.source_ref();
    let layer_reader = img_source.get_blob(&layer_digest).await?;

    log::debug!("Layer downloaded, Verifying the RootFS Layer.");
    let reader = BufReader::new(layer_reader);
    // FIXME: Use the proper decoder based on Media type
    let mut gzip_decoder = async_compression::tokio::bufread::GzipDecoder::new(reader);
    let unzipped_verify = unzipped_digest.verify(&mut gzip_decoder).await;

    if unzipped_verify {
        log::debug!("Image Layer {} verified. Saving Image Layer.", layer_digest);
        // FIXME: This unnecessarily verifies the image that we just verified above.
        let layer_reader = img_source.get_blob(&layer_digest).await?;
        let mut reader = BufReader::new(layer_reader);
        &img_layout
            .write_blob_file(&layer_digest, &mut reader)
            .await?;
    } else {
        log::error!(
            "Checksum does not match for: {} after uncompressing.",
            &layer_digest
        );
    }
    Ok(())
}
