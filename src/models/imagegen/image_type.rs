#[derive(Debug, PartialEq, PartialOrd, serde::Serialize, derive_more::Display, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ImageType {
    #[display(fmt = "bitmap")]
    Bitmap,

    // URL to an Image that should be used
    Url(String),
}

impl Default for ImageType {
    fn default() -> Self {
        Self::Bitmap
    }
}
