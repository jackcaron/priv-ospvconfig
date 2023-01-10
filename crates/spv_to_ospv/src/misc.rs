
use std::fs;
use std::io::{ Read, Write };

pub fn read_spirv_file(file_name: &String) -> std::io::Result<Vec<u8>> {
  let mut file = fs::File::open(file_name)?;
  let meta = fs::metadata(file_name)?;

  let mut buffer = Vec::<u8>::new();
  buffer.resize(meta.len() as usize, 0);

  file.read_exact(&mut buffer)?;

  Ok(buffer)
}

pub fn write_text_file(file_name: &String, data: &String) -> std::io::Result<()> {
  let mut file = fs::File::create(file_name)?;

  let buffer = data.as_bytes();
  let size = file.write(buffer)?;

  if size != buffer.len() {
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Didn't write everything"));
  }

  Ok(())
}
