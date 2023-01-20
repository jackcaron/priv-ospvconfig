
use clap::{ crate_version, load_yaml, App, ArgMatches };
use schemas::spvconfig::Spvconfig;
use std::env;
use std::ffi::OsString;
use std::io;
use std::path;

//-----------------------------------------------------
fn get_spv_config_path(matches: &ArgMatches) -> io::Result<OsString> {
  if let Some(dir) = matches.value_of("input") {
    Ok(OsString::from(dir.to_string()))
  }
  else {
    let pwd = env::current_dir()?;
    Ok(path::Path::new(&pwd).join("spvconfig.json").as_os_str().to_os_string())
  }
}

fn load_spvconfig(matches: &ArgMatches) -> io::Result<Spvconfig> {
  let config_dir = get_spv_config_path(matches)?;

  Spvconfig::from_file(&config_dir)
}

//-----------------------------------------------------
fn main() {
  let cli_yaml = load_yaml!("cli.yaml");
  let matches = App::from(cli_yaml).version(crate_version!()).get_matches();

  let spvconfig = load_spvconfig(&matches).unwrap();

  // release: matches.is_present("release")
}
