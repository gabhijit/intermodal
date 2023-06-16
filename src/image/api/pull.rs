//! Image 'pull' related APIs and internal functions

use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::sync::Arc;

use crate::image::{
    oci::{
        digest::Digest,
        layout::OCIImageLayout,
        spec_v1::{Descriptor, Image as OCIImage, Index, Manifest},
    },
    transports,
    types::ImageSource,
};
use tokio::{io::BufReader, sync::Semaphore};

/// Pulls a container image to a given Path.
///
/// Creates an OCI Image Layout rooted at the path provided. If 'force' parameter is provided and
/// the path exists, the path is overwritten, else errors out.
///
/// # Example:
///
/// ```rust
/// # use intermodal_rs::image::api::pull_container_image;
///
/// #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let temp_path = tempdir::TempDir::new("doctest.example").unwrap();
///
/// # intermodal_rs::image::transports::init_transports();
/// let result = pull_container_image("docker://busybox:latest", temp_path.path(), false, true).await;
///
/// assert!(result.is_ok())
/// # }
///
pub async fn pull_container_image<P>(
    reference: &str,
    to_path: P,
    force: bool,
    clean_on_err: bool,
) -> std::io::Result<OCIImageLayout>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    log::info!("Pulling the image: {}", reference);

    let image_ref = transports::parse_image_name(reference)?;
    let docker_ref = image_ref.docker_reference();

    if docker_ref.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid Image Name {}", reference),
        ));
    }

    let name = docker_ref.as_ref().unwrap().name();
    let tag = docker_ref.as_ref().unwrap().tag();

    log::debug!(
        "Creating OCI Image Layout for Image: {}, {}, {:?}",
        &name,
        &tag,
        to_path
    );
    let mut img_layout = OCIImageLayout::new(&name, Some(&tag), to_path);

    if img_layout.image_fs_path().exists() {
        if !force {
            let errstr = format!("Local FS path for the image with name: {}, tag: {} exists. Please specify `--force` to overwrite.", name, tag);
            log::error!("{}", errstr);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, errstr));
        } else {
            log::warn!("Local Image Layout exists, User requested 'force'. Deleting...");
            img_layout.delete_fs_path().await?;
        }
    }

    img_layout.create_fs_path().await?;

    log::debug!("Performing Image Pull.");
    let result = match perform_image_pull(&mut img_layout, reference).await {
        Ok(_) => Ok(img_layout),
        Err(e) => {
            eprintln!("Error : {}", e);
            if clean_on_err {
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

    log::trace!("Getting Manifest for the Image.");
    let manifest = img.resolved_manifest().await?;

    log::trace!("Writing Manifest Blob.");
    let digest = Digest::from_bytes(&manifest.manifest);

    let mut reader = BufReader::new(&*manifest.manifest);
    img_layout.write_blob_file(&digest, &mut reader).await?;

    let mut annotations = HashMap::new();
    // FIXME : Not sure, Also, right now we 'know' tag is `Some`.
    let _ = annotations.insert(
        "org.opencontainers.image.ref.name".to_string(),
        img_layout.tag().as_ref().unwrap().clone(),
    );

    // Manifest written, now create index.json
    let manifest_descriptor = Descriptor {
        mediatype: Some(manifest.mime_type.to_string()),
        digest,
        size: manifest.manifest.len() as i64,
        urls: None,
        platform: None,
        annotations: Some(annotations),
    };

    log::trace!("Updating Image Layout 'Index', with new manifest.");
    img_layout.update_index(Index {
        version: 2,
        manifests: vec![manifest_descriptor],
        annotations: None,
    });

    // Download and verify config
    log::trace!("Getting Image Config.");
    let manifest_obj: Manifest = serde_json::from_slice(&manifest.manifest)?;

    log::trace!("Saving Image Config.");
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
    img_source: Box<dyn ImageSource + Send + Sync>,
) -> io::Result<()> {
    log::info!("Getting Image Layer: {}", layer_digest);

    // let img_source = img.source_ref();
    let layer_reader = img_source.get_blob(&layer_digest).await?;

    log::trace!("Layer downloaded, Verifying the RootFS Layer.");
    let reader = BufReader::new(layer_reader);
    // FIXME: Use the proper decoder based on Media type
    let mut gzip_decoder = async_compression::tokio::bufread::GzipDecoder::new(reader);
    let unzipped_verify = unzipped_digest.verify(&mut gzip_decoder).await;

    if unzipped_verify {
        log::trace!("Image Layer {} verified. Saving Image Layer.", layer_digest);
        // FIXME: This unnecessarily verifies the image that we just verified above.
        let layer_reader = img_source.get_blob(&layer_digest).await?;
        let mut reader = BufReader::new(layer_reader);
        img_layout
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
