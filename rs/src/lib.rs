pub mod decode;
pub mod encode;

/// Image metadata
/// the format is always standard RGB with alpha (8 bits per channel).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwfBitmapMeta {
  /// Width in pixels
  pub width: u32,
  /// Height in pixels
  pub height: u32,
  /// Bytes per row (stride >= width * bytes_per_pixel)
  pub stride: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwfBitmap {
  pub meta: SwfBitmapMeta,
  pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
  use std::convert::TryInto;
  use std::path::Path;

  use ::image;
  use ::test_generator::test_resources;
  use image::{DynamicImage, ImageBuffer, ImageFormat, RgbaImage};
  use swf_types::tags::DefineBitmap;

  use crate::decode::decode_x_swf_bmp;
  use crate::SwfBitmap;

  #[test_resources("../tests/bitmap/swfll1/*/")]
  fn test_decode_x_swf_bmp(path: &str) {
    let path: &Path = Path::new(path);
    let _name = path
      .components()
      .last()
      .unwrap()
      .as_os_str()
      .to_str()
      .expect("Failed to retrieve sample name");
    let tag_bytes: Vec<u8> = ::std::fs::read(path.join("tag.json")).expect("Failed to read tag file");
    let tag = serde_json_v8::from_slice::<DefineBitmap>(&tag_bytes).expect("Failed to read tag");

    let actual = decode_x_swf_bmp(&tag.data).expect("Failed to decode bitmap");
    let actual_image: RgbaImage = swf_bitmap_to_rgba(&actual);

    actual_image
      .save(path.join("local-actual.rs.png"))
      .expect("Failed to write actual PNG file");

    let expected_png: Vec<u8> = ::std::fs::read(path.join("expected.png")).expect("Failed to read expected PNG file");

    let expected: DynamicImage =
      image::load_from_memory_with_format(&expected_png, ImageFormat::PNG).expect("Failed to load PNG image");

    let expected_image: RgbaImage = expected.into_rgba();
    let diff: ImageDiff = compare_images(&actual_image, &expected_image);

    if diff.error > 0 {
      diff
        .image
        .save(path.join("local-diff.rs.png"))
        .expect("Failed to write actual PNG file");
    }

    assert!(diff.same_size);
    assert_eq!(diff.error, 0);
  }

  fn swf_bitmap_to_rgba(swf_bitmap: &SwfBitmap) -> RgbaImage {
    let img: RgbaImage = ImageBuffer::from_fn(
      swf_bitmap.meta.width,
      swf_bitmap.meta.height,
      |x: u32, y: u32| -> image::Rgba<u8> {
        let x_size: usize = x.try_into().unwrap();
        let y_size: usize = y.try_into().unwrap();
        let offset: usize = swf_bitmap.meta.stride * y_size + 4 * x_size;
        let r: u8 = swf_bitmap.data[offset];
        let g: u8 = swf_bitmap.data[offset + 1];
        let b: u8 = swf_bitmap.data[offset + 2];
        let a: u8 = swf_bitmap.data[offset + 3];
        image::Rgba([r, g, b, a])
      },
    );
    img
  }

  struct ImageDiff {
    pub(crate) image: RgbaImage,
    pub(crate) same_size: bool,
    pub(crate) error: u32,
  }

  fn compare_images(actual: &RgbaImage, expected: &RgbaImage) -> ImageDiff {
    const OOB_COLOR: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);
    const ERR_COLOR: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);
    const WARN_COLOR: image::Rgba<u8> = image::Rgba([255, 255, 0, 255]);
    const OK_COLOR: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);

    let diff_width: u32 = u32::max(actual.width(), expected.width());
    let diff_height: u32 = u32::max(actual.height(), expected.height());
    let same_size = actual.width() == expected.width() && actual.height() == expected.height();
    let mut error: u32 = 0;
    let diff: RgbaImage = ImageBuffer::from_fn(diff_width, diff_height, |x: u32, y: u32| -> image::Rgba<u8> {
      let actual_px = if x < actual.width() && y < actual.height() {
        Some(actual.get_pixel(x, y))
      } else {
        None
      };
      let expected_px = if x < expected.width() && y < expected.height() {
        Some(expected.get_pixel(x, y))
      } else {
        None
      };
      let (px_error, diff_px): (u32, image::Rgba<u8>) = match (actual_px, expected_px) {
        (Some(actual_px), Some(expected_px)) => {
          let cur_err = compare_pixels(*actual_px, *expected_px);
          let color = if cur_err > 10 {
            ERR_COLOR
          } else if cur_err > 0 {
            WARN_COLOR
          } else {
            OK_COLOR
          };
          (cur_err, color)
        }
        _ => (255 * 4, OOB_COLOR),
      };

      error += px_error;
      diff_px
    });

    ImageDiff {
      image: diff,
      same_size,
      error,
    }
  }

  fn compare_pixels(actual: image::Rgba<u8>, expected: image::Rgba<u8>) -> u32 {
    let a: [u8; 4] = actual.0;
    let e: [u8; 4] = expected.0;
    let mut err: u32 = 0;
    for i in 0..4 {
      err += compare_subpixels(a[i], e[i]);
    }
    err
  }

  fn compare_subpixels(actual: u8, expected: u8) -> u32 {
    u32::from(u8::max(actual, expected) - u8::min(actual, expected))
  }
}
