
use std::fs;
use std::io::{ Write };
use std::process::{ Command };

fn generate_schema(filename: &str) {
  let src_file = format!("./definitions/{}.json", filename);
  let dst_file = format!("./src/{}", filename);

  Command::new("mkdir").args([ "-p", dst_file.as_str() ]).status().unwrap();

  let child_ospv = Command::new("jtd-codegen")
    .args([ src_file.as_str(), "--rust-out", dst_file.as_str(), "--rust-derive", "Clone" ])
    .spawn()
    .unwrap();

  println!("cargo:rerun-if-changed={}", src_file);

  let output = child_ospv.wait_with_output().unwrap();
  if !output.status.success() {
    std::process::exit(output.status.code().unwrap_or(1));
  }
}

fn write_schemas_mod(mods: &[&str]) -> std::io::Result<()> {
  let mut file = fs::File::create("src/lib.rs")?;
  file.write_all("// generated \n\n".as_bytes())?;

  for m in ["extend", "io", "spv_validate"] {
    file.write_all(format!("pub mod {};\n", m).as_bytes())?
  }

  file.write_all("\n".as_bytes())?;

  for m in mods {
    file.write_all(format!("pub mod {};\n", m).as_bytes())?
  }
  Ok(())
}

fn main() {
  let defs = [ "ospv", "spvconfig" ];
  for def in defs {
    generate_schema(def);
  }

  if let Err(e) = write_schemas_mod(&defs) {
    eprintln!("Error: {}", e);
    std::process::exit(2);
  }
}

