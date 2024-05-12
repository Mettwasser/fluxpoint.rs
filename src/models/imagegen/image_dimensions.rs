#[derive(Debug, PartialEq, PartialOrd, serde::Serialize, derive_more::Display, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ImageDimensions {
    /// Makes a perfect square
    Size(u32),

    #[display(fmt = "XY({}, {})", _0, _1)]
    /// Customizable on X and Y
    XY(u32, u32),
}

impl Default for ImageDimensions {
    fn default() -> Self {
        Self::Size(0)
    }
}
