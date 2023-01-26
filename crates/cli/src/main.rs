
use clap::{ crate_version, Parser };
use schemas::spvconfig::Spvconfig;
use std::env;
use std::ffi::OsString;
use std::io;
use std::path;

//-----------------------------------------------------
fn exit_with_err(msg: String) {
  eprintln!("ERROR:\n  {}\n", msg);
  std::process::exit(1);
}

//-----------------------------------------------------
fn get_spv_config_path(input: &Option<String>) -> io::Result<OsString> {
  if let Some(dir) = input {
    Ok(OsString::from(dir.to_string()))
  }
  else {
    let pwd = env::current_dir()?;
    Ok(path::Path::new(&pwd).join("spvconfig.json").as_os_str().to_os_string())
  }
}

fn load_spvconfig(input: &Option<String>) -> io::Result<Spvconfig> {
  let config_dir = get_spv_config_path(input)?;

  if !path::Path::new(config_dir.as_os_str()).exists() {
    exit_with_err(format!("cannot find {}", config_dir.to_str().unwrap()));
  }

  Spvconfig::from_file(&config_dir)
}

//-----------------------------------------------------
#[derive(Parser, Debug)]
#[command(name = "ospvconfig", version = crate_version!(), about = "ospvconfig compiler", long_about = None)]
struct Cli {
  /// location of "spvconfig.json" if not in current location, including file
  input: Option<String>,

  #[arg(short, long, default_value_t = false)]
  /// compile in release mode
  release: bool
}

//-----------------------------------------------------
fn main() {
  let args = Cli::parse();

  println!(">>> release mode: {}", args.release);

  let spvconfig = match load_spvconfig(&args.input) {
            Ok(data) => data,
            Err(msg) => return exit_with_err(msg.to_string())
          };

  //
}
