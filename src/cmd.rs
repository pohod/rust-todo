use crate::io;

#[derive(Debug)]
pub struct Command<'c> {
  names: Vec<&'c str>, // command may have aliases
  description: &'c str,
  action: Action
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
  Nop,
  Help,
  Quit,
  Add,
  List,
  Complete,
  Save,
  Load,
}

pub struct State<'s> {
  commands: Vec<Command<'s>>
}

impl<'s> State<'s> {
  pub fn new() -> State<'s> {
    State { commands: Vec::new() }
  }

  pub fn send_help(&self) {
    let cmd_num = self.commands.len();
    if cmd_num == 0 {
      println!("No commands.");
      return;
    }
    println!("Available commands:");
    for cmd in &self.commands {
      print!("  {}", cmd.names[0]);
      if cmd.names.len() > 1 { // aliases in (brackets)
        print!(" ({}", cmd.names[1]);
        for i in 2..cmd.names.len() {
          print!(", {}", cmd.names[i]);
        }
        print!(")");
      }
      println!(": {}", cmd.description);
    }
  }

  pub fn add_command(&mut self, names: Vec<&'s str>, description: &'s str, action: Action) {
    if names.len() == 0 { panic!("No names passed to command") }
    self.commands.push(Command { names, description, action });
  }

  pub fn accept_command(&self) -> Option<Action> {
    let cmd = match io::read_line() {
      Some(x) => x,
      None => { return Some(Action::Quit); } // EOF
    };
    if cmd.len() == 0 { return Some(Action::Nop) }
    return Some(self.find_command(&cmd as &str)?.action);
  }

  pub fn find_command(&self, name: &str) -> Option<&Command> {
    for cmd in &self.commands {
      for cmd_name in &cmd.names {
        if **cmd_name == *name { // this language is so cursed
          return Some(&cmd);
        }
      }
    }
    return None;
  }
}
