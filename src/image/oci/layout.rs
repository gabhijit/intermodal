//! Handling related to OCI Image Layout as defined here
//! https://github.com/opencontainers/image-spec/blob/master/image-layout.md

use std::error::Error as StdError;
use std::fmt::Display;
use std::path::PathBuf;

use tokio::{
    fs::OpenOptions,
    io::{AsyncWriteExt, BufWriter},
};

use super::spec_v1::{ImageLayout, Index};
use crate::utils::oci_images_root;

const OCI_LAYOUT_FILENAME: &str = "oci-layout";
const INDEX_JSON_FILENAME: &str = "index.json";

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

pub(crate) struct OCIImageLayout {
    pub(crate) name: String,
    pub(crate) image_path: PathBuf,
    pub(crate) index: Index,
    pub(crate) layout: ImageLayout,
}

impl OCIImageLayout {
    /// `OCIImageLayout` structure from the image name.
    pub fn new(name: &str) -> Result<Self, OCIImageLayoutError> {
        // It's okay to 'panic' if we can't get the base path.
        let mut image_path =
            oci_images_root().expect("Unable to get Base Directory for OCI Images.");
        let _ = image_path.push(name);

        let _ = std::fs::create_dir_all(&image_path)?;

        Ok(OCIImageLayout {
            name: name.to_string(),
            image_path,
            index: Index::default(),
            layout: ImageLayout::default(),
        })
    }

    /// Write Image Layout file.
    pub async fn write_image_layout(&self) -> Result<(), OCIImageLayoutError> {
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
    pub async fn write_index_json(&self) -> Result<(), OCIImageLayoutError> {
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_basic_layout() {
        let oci_layout = OCIImageLayout::new("foo");
        assert!(oci_layout.is_ok());
        let oci_layout = oci_layout.unwrap();

        let r = oci_layout.write_index_json().await;
        assert!(r.is_ok(), "{:#?}", r.err());

        let r = oci_layout.write_image_layout().await;
        assert!(r.is_ok(), "{:#?}", r.err());
    }
}
