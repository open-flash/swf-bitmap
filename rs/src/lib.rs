pub mod decode;
pub mod encode;

/// Image metadata
/// the format is always standard RGB with alpha (8 bits per channel).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwfBitmapMeta {
  /// Width in pixels
  pub width: usize,
  /// Height in pixels
  pub height: usize,
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
  use crate::decode::decode_x_swf_bmp;
  use crate::encode::encode_pam;
  use ::test_generator::test_resources;
  use std::path::Path;
  use swf_types::tags::DefineBitmap;

  #[test_resources("../tests/x-swf-bmp/*/")]
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

    let (_, bitmap) = decode_x_swf_bmp(&tag.data).expect("Failed to decode bitmap");

    let mut actual_pam: Vec<u8> = Vec::new();
    encode_pam(&mut actual_pam, bitmap).expect("Failed to encode PAM");

    ::std::fs::write(path.join("local-actual.rs.pam"), &actual_pam).expect("Failed to write actual PAM file");

    let expected_pam: Vec<u8> = ::std::fs::read(path.join("expected.pam")).expect("Failed to read expected PAM file");

    assert_eq!(actual_pam, expected_pam);
  }
}
