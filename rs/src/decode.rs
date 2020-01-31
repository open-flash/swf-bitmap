use crate::{SwfBitmap, SwfBitmapMeta};
use inflate::inflate_bytes_zlib;
use nom::number::complete::{le_u16 as parse_le_u16, le_u8 as parse_u8};
use nom::IResult as NomResult;

const RGBA_SIZE: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rgb {
  r: u8,
  g: u8,
  b: u8,
}

const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rgba {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

pub fn decode_x_swf_bmp(input: &[u8]) -> NomResult<&[u8], SwfBitmap> {
  use nom::combinator::map;
  use nom::multi::count;

  let (input, format_code) = parse_u8(input)?;
  if format_code != 3 {
    return Err(nom::Err::Error((input, nom::error::ErrorKind::Verify)));
  }
  let (input, width) = map(parse_le_u16, usize::from)(input)?;
  let (input, height) = map(parse_le_u16, usize::from)(input)?;
  let input_stride = width + ((4 - (width % 4)) % 4);
  let (input, color_count) = map(parse_u8, |x| usize::from(x) + 1)(input)?;

  let src_data: Vec<u8> = inflate_bytes_zlib(&input).unwrap();
  let (src_data, colors) = match count(parse_rgb, color_count)(&src_data) {
    Ok(ok) => ok,
    Err(::nom::Err::Error((_, e))) => return Err(::nom::Err::Error((&[], e))),
    Err(::nom::Err::Failure((_, e))) => return Err(::nom::Err::Failure((&[], e))),
    Err(::nom::Err::Incomplete(n)) => return Err(::nom::Err::Incomplete(n)),
  };

  let output_stride: usize = width * RGBA_SIZE;
  let mut data: Vec<u8> = Vec::with_capacity(output_stride * height);
  for y in 0..height {
    for x in 0..width {
      // TODO: Error instead of panic on out-of-bounds access
      let ci: u8 = *src_data.get(y * input_stride + x).unwrap();
      // TODO: Check how to handle out-of-bounds color indexes (currently we default to opaque black)
      let c: Rgb = *colors.get(usize::from(ci)).unwrap_or(&BLACK);
      data.extend_from_slice(&[c.r, c.g, c.b, 0xff]);
    }
  }

  Ok((
    &[],
    SwfBitmap {
      meta: SwfBitmapMeta {
        width,
        height,
        stride: output_stride,
      },
      data,
    },
  ))
}

fn parse_rgb(input: &[u8]) -> NomResult<&[u8], Rgb> {
  let (input, r) = parse_u8(input)?;
  let (input, g) = parse_u8(input)?;
  let (input, b) = parse_u8(input)?;
  Ok((input, Rgb { r, g, b }))
}
