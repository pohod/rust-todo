use crate::{cmd, io};
use std::fs;

pub struct Todo {
  pub text: String
}

pub struct State<'gs> {
  pub cmds: cmd::State<'gs>,
  pub todos: Vec<Todo>,
  dirty: bool
}

impl<'gs> State<'gs> {
  pub fn new() -> State<'gs> {
    State { cmds: cmd::State::new(), todos: Vec::new(), dirty: false }
  }

  fn confirm_quit() -> Result<(), String> {
    println!("You have unsaved TODOs, really quit?");
    print!("(y/N): ");
    io::flush_out();
    let resp = io::read_line().ok_or("n".to_string())?.to_string().to_lowercase();
    if resp.chars().nth(0) != Some('y') {
      println!("Aborted.");
      println!("TIP: save TODOs to a file with the 's' command");
      return Ok(());
    }
    return Err("Quitting.".to_string());
  }

  pub fn maybe_quit(&self) -> Result<(), String> {
    return if self.dirty { State::confirm_quit() } else { Err("Quitting.".to_string()) }
  }

  pub fn add_todo_prompt(&mut self) {
    print!("TODO Text: ");
    io::flush_out();
    match io::read_line() {
      None => println!("Aborted."),
      Some(line) => {
        if line.len() == 0 { println!("Aborted.") }
        else { self.add_todo(line) }
      }
    }
  }

  pub fn add_todo(&mut self, text: String) {
    self.dirty = true;
    println!("Added todo #{}: {text}", self.todos.len());
    self.todos.push(Todo { text: text.clone() });
  }

  pub fn list_todos(&self) {
    println!("All TODOs:");
    if self.todos.len() > 0 {
      for (i, todo) in self.todos.iter().enumerate() {
        println!("  ({i}): {}", todo.text);
      }
    } else {
      println!("No TODOs, yay!");
    }
  }

  pub fn complete_todo_prompt(&mut self) {
    print!("TODO ID to remove: ");
    io::flush_out();
    match io::read_line() {
      None => println!("Aborted."),
      Some(line) => {
        let idx = match line.parse::<usize>() {
          Ok(num) => num,
          Err(e) => { println!("Error: index invalid: {e}"); return; }
        };
        if idx >= self.todos.len() {
          println!("Error: index too large");
        } else {
          self.complete_todo(idx);
        }
      }
    }
  }

  pub fn complete_todo(&mut self, index: usize) {
    self.dirty = true;
    let todo = self.todos.remove(index);
    println!("Removed TODO #{index}: {}", todo.text);
  }

  pub fn save_todos(&mut self) -> Result<(), String> {
    let save_file = io::prompt("Save file: ").unwrap();
    let mut buf = String::new();
    for todo in &self.todos {
      buf.push_str(&todo.text);
      buf.push_str("\n");
    }
    fs::write(save_file, buf).expect("Failed to write file");
    self.dirty = false;
    Ok(())
  }

  pub fn load_todos(&mut self) -> Result<(), String> {
    if self.todos.len() > 0 {
      println!("NOTE: there are TODOs already present, they will not be overwritten.");
    }
    let save_file = io::prompt("TODO file: ").unwrap();
    let raw_vec = match fs::read(&save_file) {
      Ok(x) => x,
      Err(e) => {
        println!("Error while reading '{save_file}': {}", e);
        println!("Aborting.");
        return Ok(());
      }
    };
    let buf = String::from_utf8(raw_vec).unwrap();
    let mut loaded_num: usize = 0;
    for todo in buf.split('\n') {
      let todo_text = todo.trim();
      if todo_text.len() == 0 { continue }; // skip
      self.todos.push(Todo {text: todo_text.to_string()});
      loaded_num += 1;
    }

    println!("Loaded {loaded_num} TODOs from {save_file}!");
    self.dirty = false;
    Ok(())
  }
}
