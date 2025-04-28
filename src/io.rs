use std::io::{stdout, Write};

pub fn flush_out() {
  stdout().flush().unwrap();
}

pub fn prompt(msg: &str) -> Option<String> {
  print!("{}", msg);
  flush_out();
  return read_line();
}

pub fn read_line() -> Option<String> {
  Some(read_line_no_trim()?.trim().to_string())
}

fn read_line_no_trim() -> Option<String> {
  let mut buf = String::new();
  let read_bytes = std::io::stdin().read_line(&mut buf).unwrap();
  return if read_bytes == 0 { None } else { Some(buf) }
}
