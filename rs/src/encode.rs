use crate::SwfBitmap;
use std::io;

pub fn encode_pam<W: io::Write>(writer: &mut W, bitmap: SwfBitmap) -> io::Result<()> {
  writer.write_all(b"P7\n")?;
  writer.write_all(format!("WIDTH {}\n", bitmap.meta.width).as_bytes())?;
  writer.write_all(format!("HEIGHT {}\n", bitmap.meta.height).as_bytes())?;
  writer.write_all(b"DEPTH 4\n")?;
  writer.write_all(b"MAXVAL 255\n")?;
  writer.write_all(b"TUPLTYPE RGB_ALPHA\n")?;
  writer.write_all(b"ENDHDR\n")?;
  writer.write_all(&bitmap.data)?;
  Ok(())
}
