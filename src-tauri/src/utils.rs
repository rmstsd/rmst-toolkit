use std::io::Read;
use std::{fs::File, io};

pub fn readFile(path: &str) -> io::Result<String> {
  let mut f = File::open(path)?;
  let mut buffer: String = String::new();

  f.read_to_string(&mut buffer)?;

  Ok(buffer)
}
