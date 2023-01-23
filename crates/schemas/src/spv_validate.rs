use crate::spvconfig::{ Compiler, IoFormat, IoFormatFormat, IoFormatPrefix };

type CompilerResult = Result<(), String>;

pub enum BuildTarget {
  Debug,
  Release
}

//-----------------------------------------------------
fn require_single_pattern(data: &String, patt: &str) -> CompilerResult {
  match data.as_str().matches(patt).count() {
    0 => Err(format!("missing '{}'", patt)),
    1 => Ok(()),
    _ => Err(format!("contains multiple '{}'", patt))
  }
}

fn at_most_one_pattern(data: &String, patt: &str) -> CompilerResult {
  match data.as_str().matches(patt).count() {
    0 |
    1 => Ok(()),
    _ => Err(format!("contains multiple '{}'", patt))
  }
}

//-----------------------------------------------------
impl IoFormatFormat {
  fn validate(&self) -> CompilerResult {
    require_single_pattern(&self.format0, "{}").map_err(|x| { format!("format {}", x) })
  }

  fn format(&self, file_name: &str) -> String {
    self.format0.replace("{}", file_name)
  }
}

impl IoFormatPrefix {
  fn validate(&self) -> CompilerResult {
    match self.prefix.is_empty() {
      false => Ok(()),
      true => Err("prefix is empty".to_string())
    }
  }

  fn format(&self, file_name: &str) -> String {
    format!("{} {}", self.prefix, file_name)
  }
}

impl IoFormat {
  fn validate(&self) -> CompilerResult {
    match self {
      IoFormat::Empty(_) => Ok(()),
      IoFormat::Prefix(p) => p.validate(),
      IoFormat::Format0(f) => f.validate()
    }
  }

  fn format(&self, file_name: &str) -> String {
    match self {
      IoFormat::Empty(_) => file_name.to_string(),
      IoFormat::Prefix(p) => p.format(file_name),
      IoFormat::Format0(f) => f.format(file_name)
    }
  }
}

//-----------------------------------------------------
fn spawn_err(msg: String) -> String {
  format!("spawn {}", msg)
}

fn check_spawn(spawn: &String) -> CompilerResult {
  if spawn.is_empty() {
    return Err(spawn_err("is_empty".to_string()));
  }

  require_single_pattern(spawn, "{input}").map_err(spawn_err)?;
  require_single_pattern(spawn, "{output}").map_err(spawn_err)?;

  at_most_one_pattern(spawn, "{options}").map_err(spawn_err)?;
  at_most_one_pattern(spawn, "{debug_options}").map_err(spawn_err)?;
  at_most_one_pattern(spawn, "{release_options}").map_err(spawn_err)?;

  Ok(())
}

fn spawn_option(opt: &Option<Box<String>>) -> String {
  if opt.is_none() {
    String::new()
  }
  else {
    opt.as_ref().map(|x|{ x.as_ref().clone() }).unwrap()
  }
}

impl Compiler {
  fn err(&self, msg: &str) -> String {
    format!("compiler {} {}", self.name, msg)
  }

  fn prepare_spawn(&self, target: BuildTarget) -> String {
    (
      match target {
        BuildTarget::Debug => self.spawn.replace("{debug_options}", spawn_option(&self.debug_options).as_str()),
        BuildTarget::Release => self.spawn.replace("{release_options}", spawn_option(&self.release_options).as_str())
      }
    ).replace("{options}", spawn_option(&self.options).as_str())
  }

  pub fn validate(&self) -> CompilerResult {
    if self.name.is_empty() {
      return Err("compiler has empty name".to_string());
    }

    if self.executable.is_empty() {
      return Err(self.err("has empty executable"));
    }

    let err_map = |x:String| { self.err(&x) };

    check_spawn(&self.spawn).map_err(err_map)?;
    self.input.validate().map_err(err_map)?;
    self.output.validate().map_err(err_map)?;

    Ok(())
  }

  pub fn prepare(&self, target: BuildTarget) -> Compiler {
    Compiler {
      executable: self.executable.clone(),
      input: self.input.clone(),
      name: self.name.clone(),
      output: self.output.clone(),
      spawn: self.prepare_spawn(target),
      debug_options: None,
      options: None,
      release_options: None
    }
  }

  pub fn format(&self, input_file: &str, output_file: &str) -> String {
    self.spawn.replace("{input}", &self.input.format(input_file).as_str())
              .replace("{output}", &self.output.format(output_file).as_str())
  }
}

