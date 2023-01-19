
use clap::{crate_version, load_yaml, App};

fn main() {
  let cli_yaml = load_yaml!("cli.yaml");
  let matches = App::from(cli_yaml).version(crate_version!()).get_matches();

  //
}

