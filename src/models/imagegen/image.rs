use crate::imagegen::{ImageType, __NoImageDimensions, __NoImageType};

use super::image_builder::{ImageBuilder, __NoColor};

#[derive(Debug, PartialEq, PartialOrd, serde::Serialize)]
pub struct Image {
    pub(crate) image_type: ImageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) size: Option<u32>,
    pub(crate) color: String,
    pub(crate) round: u32,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) skip: bool,
}

impl Image {
    pub fn builder() -> ImageBuilder<__NoImageType, __NoImageDimensions, __NoColor> {
        ImageBuilder::new()
    }
}
