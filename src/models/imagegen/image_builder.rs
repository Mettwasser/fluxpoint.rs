use crate::{
    imagegen::{Image, ImageDimensions, ImageType},
    models::Color,
};

use super::macros::state;

state! {
    ImageType;
    ImageDimensions;
    String : Color;
}

#[derive(Default, Debug, PartialEq, PartialOrd, serde::Serialize)]
pub struct ImageBuilder<ImageType, Dimensions, Color> {
    image_type: ImageType,

    dimensions: Dimensions,

    color: Color,
    round: u32,
    x: u32,
    y: u32,
    skip: bool,
}

impl ImageBuilder<__NoImageType, __NoImageDimensions, __NoColor> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn image_type(
        self,
        image_type: ImageType,
    ) -> ImageBuilder<__ImageType, __NoImageDimensions, __NoColor> {
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

    pub fn dimensions(
        self,
        dimensions: ImageDimensions,
    ) -> ImageBuilder<__NoImageType, __ImageDimensions, __NoColor> {
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

impl ImageBuilder<__NoImageType, __ImageDimensions, __NoColor> {
    pub fn image_type(
        self,
        image_type: ImageType,
    ) -> ImageBuilder<__ImageType, __ImageDimensions, __NoColor> {
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

impl ImageBuilder<__ImageType, __NoImageDimensions, __NoColor> {
    pub fn dimensions(
        self,
        dimensions: ImageDimensions,
    ) -> ImageBuilder<__ImageType, __ImageDimensions, __NoColor> {
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

impl ImageBuilder<__ImageType, __ImageDimensions, __NoColor> {
    pub fn color(self, color: Color) -> ImageBuilder<__ImageType, __ImageDimensions, __Color> {
        let color = color.to_string();
        ImageBuilder {
            color: __Color(color),
            dimensions: self.dimensions,
            image_type: self.image_type,
            round: self.round,
            skip: self.skip,
            x: self.x,
            y: self.y,
        }
    }

    pub fn build(self) -> Image {
        let mut width = None;
        let mut height = None;
        let mut size = None;

        match self.dimensions.0 {
            ImageDimensions::Size(n) => size = Some(n),
            ImageDimensions::XY(x, y) => (width, height) = (Some(x), Some(y)),
        }

        Image {
            image_type: self.image_type.0,
            width,
            height,
            size,
            color: String::new(),
            round: self.round,
            x: self.x,
            y: self.y,
            skip: self.skip,
        }
    }
}

impl<C> ImageBuilder<__ImageType, __ImageDimensions, C> {
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

        Image {
            image_type: self.image_type.0,
            width,
            height,
            size,
            color: self.color.0,
            round: self.round,
            x: self.x,
            y: self.y,
            skip: self.skip,
        }
    }
}

#[cfg(test)]
mod test {

    use serde_json::json;

    use crate::models::Color;

    use crate::models::imagegen::{Image, ImageType};

    use super::ImageDimensions;

    #[test]
    fn test_image_builder() {
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

    #[test]
    fn test_image_builder_json() {
        let image = Image::builder()
            .dimensions(ImageDimensions::Size(69))
            .image_type(ImageType::Bitmap)
            .color(Color::RGB(255, 0, 0))
            .build();

        let x = Image::builder().dimensions(ImageDimensions::Size(31)).image_type(ImageType::Bitmap).

        assert_eq!(
            serde_json::to_value(image).unwrap(),
            json!({"image_type": "bitmap", "size": 69, "color": "255,0,0", "round": 0, "x": 0, "y": 0, "skip": false})
        );
    }
}
