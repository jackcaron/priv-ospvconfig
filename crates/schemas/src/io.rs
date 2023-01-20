
use serde_json;
use std::ffi::OsString;
use std::fs;
use std::io::{ Error, ErrorKind, Read, Write };

use crate::ospv::{ Ospv };
use crate::spvconfig::{ Spvconfig };

//-----------------------------------------------------
pub fn read_file(file_name: &OsString) -> std::io::Result<Vec<u8>> {
  let meta = fs::metadata(file_name)?;
  let mut file = fs::File::open(file_name)?;

  let mut buffer = Vec::<u8>::new();
  buffer.resize(meta.len() as usize, 0);

  file.read_exact(&mut buffer)?;

  Ok(buffer)
}

fn read_text_file(file_name: &OsString) -> std::io::Result<String> {
  let mut file = fs::File::open(file_name)?;

  let mut buf = String::new();
  file.read_to_string(&mut buf)?;

  Ok(buf)
}

fn write_text_file(file_name: &OsString, data: &String) -> std::io::Result<()> {
  let mut file = fs::File::create(file_name)?;

  let buffer = data.as_bytes();
  let size = file.write(buffer)?;

  if size != buffer.len() {
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Didn't write everything"));
  }

  Ok(())
}

//-----------------------------------------------------
// schema implementations
macro_rules! SchemaExt {
  ($val: ty) => {
    impl $val {
      // ---------------------
      pub fn from_json_str(json_str: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json_str)
      }

      pub fn from_file(file_name: &OsString) -> std::io::Result<Self> {
        let raw = read_text_file(file_name)?;

        match serde_json::from_str(&raw) {
          Ok(slf) => Ok(slf),
          Err(err) => Err(Error::new(ErrorKind::Other, format!("JSON ERROR:\n{}\n", err.to_string())))
        }
      }

      // ---------------------
      pub fn to_json_str(&self) -> serde_json::Result<String> {
        if cfg!(debug_assertions) {
          serde_json::to_string_pretty(&self)
        }
        else {
          serde_json::to_string(&self)
        }
      }

      pub fn to_file(&self, file_name: &OsString) -> std::io::Result<()> {
        let data = match self.to_json_str() {
          Ok(data) => data,
          Err(err) => return Err(Error::new(ErrorKind::Other, format!("JSON ERROR:\n{}\n", err.to_string())))
        };
        write_text_file(file_name, &data)
      }
    }
  };
}

SchemaExt!(Ospv);
SchemaExt!(Spvconfig);
