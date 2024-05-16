use crate::imagegen::ImageType;

#[derive(Default, Debug, PartialEq, PartialOrd, serde::Serialize)]
pub struct ImageBase {
    pub(crate) image_type: ImageType,
    pub(crate) color: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
}
