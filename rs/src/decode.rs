use crate::{SwfBitmap, SwfBitmapMeta};
use core::convert::TryFrom;
use core::mem::size_of;
use inflate::inflate_bytes_zlib;
use nom::number::complete::{le_u16 as parse_le_u16, le_u8 as parse_u8};
use nom::IResult as NomResult;

const RGBA_SIZE: usize = size_of::<[u8; 4]>();

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rgb {
  r: u8,
  g: u8,
  b: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rgba {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

/// Byte alignment for a row of data
const ROW_ALIGNMENT: usize = 4;

// TODO: Add variants
// TODO: Implement error trait
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecodeError {
  Invalid,
}

pub fn decode_x_swf_bmp(input: &[u8]) -> Result<SwfBitmap, DecodeError> {
  let (input, header) = match parse_lossless_header(input) {
    Ok(ok) => ok,
    Err(_) => return Err(DecodeError::Invalid),
  };

  match header.code {
    0x03 => {
      let (input, color_count) = match parse_u8::<()>(input) {
        Ok(ok) => ok,
        Err(_) => return Err(DecodeError::Invalid),
      };
      match inflate_bytes_zlib(&input) {
        Ok(data) => decode_color_map(&data, header.width, header.height, usize::from(color_count) + 1),
        Err(_) => Err(DecodeError::Invalid),
      }
    }
    0x04 => match inflate_bytes_zlib(&input) {
      Ok(data) => decode_pix_map15(&data, header.width, header.height),
      Err(_) => Err(DecodeError::Invalid),
    },
    0x05 => match inflate_bytes_zlib(&input) {
      Ok(data) => decode_pix_map24(&data, header.width, header.height),
      Err(_) => Err(DecodeError::Invalid),
    },
    _ => Err(DecodeError::Invalid),
  }
}

struct LosslessHeader {
  code: u8,
  width: u16,
  height: u16,
}

fn parse_lossless_header(input: &[u8]) -> NomResult<&[u8], LosslessHeader> {
  let (input, code) = parse_u8(input)?;
  let (input, width) = parse_le_u16(input)?;
  let (input, height) = parse_le_u16(input)?;
  Ok((input, LosslessHeader { code, width, height }))
}

fn decode_color_map(input: &[u8], width: u16, height: u16, color_count: usize) -> Result<SwfBitmap, DecodeError> {
  use nom::multi::count;
  let (input, colors) = match count(parse_rgb, color_count)(&input) {
    Ok(ok) => ok,
    Err(_) => return Err(DecodeError::Invalid),
  };

  const INPUT_PIXEL_SIZE: usize = size_of::<u8>();
  let input_stride = int_ceil(usize::from(width) * INPUT_PIXEL_SIZE, ROW_ALIGNMENT);
  if input.len() < input_stride * usize::from(height) {
    return Err(DecodeError::Invalid);
  }

  let stride: usize = usize::from(width) * RGBA_SIZE;
  let mut data: Vec<u8> = Vec::with_capacity(stride * usize::from(height));
  for y in 0..usize::from(height) {
    for x in 0..usize::from(width) {
      let offset = input_stride * y + INPUT_PIXEL_SIZE * x;
      let color_index: usize = input[offset].into();
      // TODO: Check how to handle out-of-bounds color indexes
      match colors.get(color_index) {
        Some(c) => data.extend_from_slice(&[c.r, c.g, c.b, 0xff]),
        None => return Err(DecodeError::Invalid),
      };
    }
  }
  Ok(SwfBitmap {
    meta: SwfBitmapMeta {
      width: width.into(),
      height: height.into(),
      stride,
    },
    data,
  })
}

fn decode_pix_map15(input: &[u8], width: u16, height: u16) -> Result<SwfBitmap, DecodeError> {
  const INPUT_PIXEL_SIZE: usize = size_of::<u16>();
  let input_stride = int_ceil(usize::from(width) * INPUT_PIXEL_SIZE, ROW_ALIGNMENT);
  if input.len() < input_stride * usize::from(height) {
    return Err(DecodeError::Invalid);
  }
  let stride: usize = usize::from(width) * RGBA_SIZE;
  let mut data: Vec<u8> = Vec::with_capacity(stride * usize::from(height));
  for y in 0..usize::from(height) {
    for x in 0..usize::from(width) {
      let offset = input_stride * y + INPUT_PIXEL_SIZE * x;
      let pix15: u16 = (u16::from(input[offset]) << 8) + u16::from(input[offset + 1]);
      let c = decode_pix15(pix15);
      data.extend_from_slice(&[c.r, c.g, c.b, 0xff]);
    }
  }
  Ok(SwfBitmap {
    meta: SwfBitmapMeta {
      width: width.into(),
      height: height.into(),
      stride,
    },
    data,
  })
}

fn decode_pix15(pixel: u16) -> Rgb {
  fn decode_component(px: u16, idx: u16) -> u8 {
    u8::try_from((px >> (5 * idx)) & 0x1f).unwrap() << 3
  }

  let r: u8 = decode_component(pixel, 2);
  let g: u8 = decode_component(pixel, 1);
  let b: u8 = decode_component(pixel, 0);
  Rgb { r, g, b }
}

fn decode_pix_map24(input: &[u8], width: u16, height: u16) -> Result<SwfBitmap, DecodeError> {
  const INPUT_PIXEL_SIZE: usize = size_of::<[u8; 4]>();
  let input_stride = int_ceil(usize::from(width) * INPUT_PIXEL_SIZE, ROW_ALIGNMENT);
  if input.len() < input_stride * usize::from(height) {
    return Err(DecodeError::Invalid);
  }
  let stride: usize = usize::from(width) * RGBA_SIZE;
  let mut data: Vec<u8> = Vec::with_capacity(stride * usize::from(height));
  for y in 0..height {
    for x in 0..width {
      let offset = input_stride * usize::from(y) + INPUT_PIXEL_SIZE * usize::from(x);
      // `input[offset + 0]` is reserved
      let r: u8 = input[offset + 1];
      let g: u8 = input[offset + 2];
      let b: u8 = input[offset + 3];
      data.extend_from_slice(&[r, g, b, 0xff]);
    }
  }
  Ok(SwfBitmap {
    meta: SwfBitmapMeta {
      width: width.into(),
      height: height.into(),
      stride,
    },
    data,
  })
}

/// Returns the smallest multiple of `k` greater than or equal to `n`.
fn int_ceil(n: usize, k: usize) -> usize {
  assert!(k > 0);
  let padding: usize = k - 1 - (n.wrapping_sub(1) % k);
  n + padding
}

fn parse_rgb(input: &[u8]) -> NomResult<&[u8], Rgb> {
  let (input, r) = parse_u8(input)?;
  let (input, g) = parse_u8(input)?;
  let (input, b) = parse_u8(input)?;
  Ok((input, Rgb { r, g, b }))
}

#[cfg(test)]
mod tests {
  use super::int_ceil;

  #[test]
  fn test_int_ceil() {
    assert_eq!(int_ceil(0, 4), 0);
    assert_eq!(int_ceil(1, 4), 4);
    assert_eq!(int_ceil(2, 4), 4);
    assert_eq!(int_ceil(3, 4), 4);
    assert_eq!(int_ceil(4, 4), 4);
    assert_eq!(int_ceil(5, 4), 8);
    assert_eq!(int_ceil(6, 4), 8);
    assert_eq!(int_ceil(7, 4), 8);
    assert_eq!(int_ceil(8, 4), 8);
    assert_eq!(int_ceil(252, 4), 252);
    assert_eq!(int_ceil(253, 4), 256);
    assert_eq!(int_ceil(254, 4), 256);
    assert_eq!(int_ceil(255, 4), 256);
    assert_eq!(int_ceil(256, 4), 256);
  }
}
