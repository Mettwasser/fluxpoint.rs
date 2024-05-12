use crate::{
    imagegen::{Image, ImageDimensions, ImageType},
    models::Color,
};

use super::macros::state;

state! {
    ImageType;
    ImageDimensions;
}

#[derive(Default, Debug, PartialEq, PartialOrd, serde::Serialize)]
pub struct ImageBuilder<ImageType, Dimensions> {
    image_type: ImageType,

    dimensions: Dimensions,

    color: Option<Color>,
    round: u32,
    x: u32,
    y: u32,
    skip: bool,
}

impl ImageBuilder<__NoImageType, __NoImageDimensions> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn image_type(
        self,
        image_type: ImageType,
    ) -> ImageBuilder<__ImageType, __NoImageDimensions> {
        ImageBuilder {
            image_type: __ImageType(image_type),
            color: self.color,
            dimensions: self.dimensions,
            round: self.round,
            skip: self.skip,
            x: self.x,
            y: self.y,
        }
    }
}

impl ImageBuilder<__ImageType, __NoImageDimensions> {
    pub fn dimensions(
        self,
        dimensions: ImageDimensions,
    ) -> ImageBuilder<__ImageType, __ImageDimensions> {
        ImageBuilder {
            dimensions: __ImageDimensions(dimensions),
            image_type: self.image_type,
            color: self.color,
            round: self.round,
            skip: self.skip,
            x: self.x,
            y: self.y,
        }
    }
}

impl ImageBuilder<__ImageType, __ImageDimensions> {
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn x(mut self, n: u32) -> Self {
        self.x = n;
        self
    }
    pub fn y(mut self, n: u32) -> Self {
        self.y = n;
        self
    }
    pub fn skip(mut self, skip: bool) -> Self {
        self.skip = skip;
        self
    }

    pub fn build(self) -> Image {
        let mut width = None;
        let mut height = None;
        let mut size = None;

        match self.dimensions.0 {
            ImageDimensions::Size(n) => size = Some(n),
            ImageDimensions::XY(x, y) => (width, height) = (Some(x), Some(y)),
        }

        let color = if let Some(color) = self.color {
            color.to_string()
        } else {
            String::new()
        };

        Image {
            image_type: self.image_type.0,
            width,
            height,
            size,
            color,
            round: self.round,
            x: self.x,
            y: self.y,
            skip: self.skip,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::models::Color;

    use crate::models::imagegen::{Image, ImageType};

    use super::ImageDimensions;

    #[test]
    fn test_builder() {
        let image = Image::builder()
            .image_type(ImageType::Bitmap)
            .dimensions(ImageDimensions::Size(69))
            .color(Color::RGB(255, 0, 0))
            .build();

        assert_eq!(
            image,
            Image {
                color: String::from("255,0,0"),
                height: None,
                width: None,
                size: Some(69),
                image_type: ImageType::Bitmap,
                round: 0,
                x: 0,
                y: 0,
                skip: false
            }
        )
    }
}
