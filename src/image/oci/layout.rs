//! Handling related to OCI Image Layout as defined here
//! https://github.com/opencontainers/image-spec/blob/master/image-layout.md

//! # Notes
//!
//! A container image will be stored on a file system using the layout as discussed in the
//! document above. For our use cases, we'll be storing an image inside a directory on a file
//! system under a path that looks like `<BASE_DIR>/<name>/[<tag>]/`. This allows us to store
//! images separately for individual tags, each with their own 'layout'. The `tag` is optional,
//! when creating a local image layout, when images are copied from a Docker reference say, if the
//! tag is implicit (like `latest`), it will be used. In general this should address all the use
//! cases. One particular issue is - what if a user tries to write to a layout that already exists.
//! This is not extremely uncommon (though not very common as well.) Let's say there already exists
//! an image in the FS (downloaded as part of `copy` or `pull`) and next action is going to
//! overwrite this. The best is to simply warn and provide a force option to overwrite, deleting
//! existing (`index.json` and perhaps some `blobs` as well.)
//!

use std::error::Error as StdError;
use std::fmt::Display;
use std::path::PathBuf;

use tokio::{
    fs::{File, OpenOptions},
    io::{self, AsyncRead, AsyncWriteExt, BufWriter},
};

use super::{
    digest::Digest,
    spec_v1::{ImageLayout, Index},
};
use crate::utils::oci_images_root;

const OCI_LAYOUT_FILENAME: &str = "oci-layout";
const INDEX_JSON_FILENAME: &str = "index.json";
const BLOBS_DIRNAME: &str = "blobs";

#[derive(Debug)]
pub struct OCIImageLayoutError(String);

impl StdError for OCIImageLayoutError {}

impl Display for OCIImageLayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Layout Error: {}", self.0)
    }
}

impl From<std::io::Error> for OCIImageLayoutError {
    fn from(e: std::io::Error) -> Self {
        OCIImageLayoutError(format!("{}", e))
    }
}

impl From<serde_json::Error> for OCIImageLayoutError {
    fn from(e: serde_json::Error) -> Self {
        OCIImageLayoutError(format!("{}", e))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OCIImageLayout {
    pub(crate) name: String,
    pub(crate) tag: Option<String>,
    pub(crate) image_path: PathBuf,
    pub(crate) index: Index,
    pub(crate) layout: ImageLayout,
    pub(crate) fs_path_exists: bool,
}

impl OCIImageLayout {
    /// `OCIImageLayout` structure from the image name and optional tag.
    pub fn new(name: &str, tag: Option<&str>, path: Option<&PathBuf>) -> Self {
        // It's okay to 'panic' if we can't get the base path.
        let mut image_path = match path {
            Some(p) => PathBuf::from(p),
            None => oci_images_root().expect("Unable to get Base Directory for OCI Images."),
        };

        if tag.is_none() {
            let _ = image_path.push(name);
        } else {
            let _ = image_path.push(format!("{}/{}", name, tag.unwrap()));
        }

        let tag = match tag {
            Some(t) => Some(t.to_string()),
            None => None,
        };

        OCIImageLayout {
            name: name.to_string(),
            tag,
            index: Index::default(),
            layout: ImageLayout::default(),
            fs_path_exists: image_path.exists(),
            image_path,
        }
    }

    /// Create the Layout on the FS
    ///
    /// Creates the underlying 'blobs' directory as well (As it is a required one.)
    pub async fn create_fs_path(&mut self) -> Result<(), std::io::Error> {
        let mut path = self.image_path.clone();
        path.push(BLOBS_DIRNAME);
        let _ = tokio::fs::create_dir_all(&path).await?;
        self.fs_path_exists = true;

        Ok(())
    }

    /// Delete the Layout from the FS
    pub async fn delete_fs_path(&mut self) -> Result<(), std::io::Error> {
        let _ = tokio::fs::remove_dir_all(&self.image_path).await?;
        self.fs_path_exists = false;

        Ok(())
    }

    /// Write Image Layout file.
    pub async fn write_image_layout(&self) -> Result<(), std::io::Error> {
        let mut layout_file_path = self.image_path.clone();
        layout_file_path.push(OCI_LAYOUT_FILENAME);

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(layout_file_path)
            .await?;

        let contents = serde_json::to_vec(&self.layout)?;
        let mut writer = BufWriter::new(file);
        writer.write(&contents).await?;
        writer.flush().await?;

        Ok(())
    }

    /// Write Image `index.json` file
    pub async fn write_index_json(&self) -> Result<(), std::io::Error> {
        let mut index_json_path = self.image_path.clone();
        index_json_path.push(INDEX_JSON_FILENAME);

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(index_json_path)
            .await?;

        let contents = serde_json::to_vec(&self.index)?;
        let mut writer = BufWriter::new(file);
        writer.write(&contents).await?;
        writer.flush().await?;

        Ok(())
    }

    /// Write a blob file
    ///
    /// The digest specifies the <algorithm>/<filename> part
    pub async fn write_blob_file<T>(
        &self,
        digest: &Digest,
        blob: &mut T,
    ) -> Result<(), std::io::Error>
    where
        T: AsyncRead + Unpin,
    {
        let mut path = self.image_path.clone();
        path.push(BLOBS_DIRNAME);
        path.push(digest.algorithm());
        if !path.exists() {
            tokio::fs::create_dir(&path).await?;
        }

        let _ = path.push(digest.hex_digest());

        let mut file = File::create(&path).await?;

        io::copy(blob, &mut file).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_basic_layout() {
        let mut oci_layout = OCIImageLayout::new("foo", None, None);

        let r = oci_layout.create_fs_path().await;
        assert!(r.is_ok());

        let r = oci_layout.write_index_json().await;
        assert!(r.is_ok(), "{:#?}", r.err());

        let r = oci_layout.write_image_layout().await;
        assert!(r.is_ok(), "{:#?}", r.err());

        let r = oci_layout.delete_fs_path().await;
        assert!(r.is_ok());
    }
}
