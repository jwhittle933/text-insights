use std::io::Cursor;

use anyhow::Context;
use image::GrayImage;
use imageproc::distance_transform::Norm;

pub struct Image {
    img: GrayImage,
}

impl Image {
    pub const THRESHOLD_BLOCK_RADIUS: u32 = 64;
    pub const MEDIAN_FILTER: u32 = 10;
    pub const BLUR_SIGMA: f32 = 0.5;
    pub const DILATE_K: u8 = 1;

    fn median_filter(mut self, x: Option<u32>, y: Option<u32>) -> Self {
        self.img = imageproc::filter::median_filter(
            &self.img,
            x.unwrap_or(Self::MEDIAN_FILTER),
            y.unwrap_or(Self::MEDIAN_FILTER),
        );

        self
    }

    fn adaptive_threshold(mut self, r: Option<u32>) -> Self {
        self.img = imageproc::contrast::adaptive_threshold(
            &self.img,
            r.unwrap_or(Self::THRESHOLD_BLOCK_RADIUS),
        );
        self
    }

    fn blur(mut self, sigma: Option<f32>) -> Self {
        self.img = image::imageops::fast_blur(&self.img, sigma.unwrap_or(Self::BLUR_SIGMA));
        self
    }

    fn dilate(mut self, k: Option<u8>) -> Self {
        imageproc::morphology::dilate_mut(&mut self.img, Norm::L1, k.unwrap_or(Self::DILATE_K));
        self
    }

    pub fn transform(self) -> Self {
        self.median_filter(None, None)
            .adaptive_threshold(None)
            .blur(None)
            .dilate(None)
    }

    pub fn into_image(self) -> GrayImage {
        self.img
    }

    pub fn debug_write(&self, label: &str, fname: &str) -> anyhow::Result<()> {
        #[cfg(debug_assertions)]
        {
            let debug_image = image::DynamicImage::ImageLuma8(self.img.clone());
            let dir = format!(".data/image/processed/{label}");
            std::fs::create_dir_all(&dir).context("failed to create directories")?;

            println!("Writing to {dir}/{fname}");
            debug_image.save(format!("{dir}/{fname}"))?;
        }

        Ok(())
    }
}

impl TryFrom<Vec<u8>> for Image {
    type Error = anyhow::Error;

    fn try_from(data: Vec<u8>) -> anyhow::Result<Self> {
        let mut reader = image::ImageReader::new(Cursor::new(data));
        reader.set_format(image::ImageFormat::Jpeg);
        let img = reader.decode().context("failed to decode image")?;
        Ok(Self {
            img: img.to_luma8(),
        })
    }
}
