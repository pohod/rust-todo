mod state;
use state::{State};

mod cmd;
use cmd::Action;

mod io;

fn run_cmd(state: &mut State, act: Action) -> Result<(), String> {
  match act {
    Action::Nop => Ok(()), // do nothing
    Action::Quit => state.maybe_quit(),
    Action::Help => { state.cmds.send_help(); Ok(()) },
    Action::Add => { state.add_todo_prompt(); Ok(()) },
    Action::List => { state.list_todos(); Ok(()) },
    Action::Complete => { state.complete_todo_prompt(); Ok(()) },

    Action::Save => state.save_todos(),
    Action::Load => state.load_todos()
  }
}

fn main() {
  let mut state = State::new();
  state.cmds.add_command(vec!("q"), "Exit", Action::Quit);
  state.cmds.add_command(vec!("a", "add"), "Add a TODO", Action::Add);
  state.cmds.add_command(vec!("l", "list"), "List all TODO", Action::List);
  state.cmds.add_command(vec!("r", "remove", "c", "comp", "delete"), "Remove/Complete a TODO", Action::Complete);
  state.cmds.add_command(vec!("s", "save"), "Save all TODOs to a file", Action::Save);
  state.cmds.add_command(vec!("load", "ld"), "Load TODOs from a file", Action::Load);
  state.cmds.add_command(vec!("?", "h", "help"), "Print Help", Action::Help);
  println!("Enter a command (? for help):");
  loop {
    print!("> ");
    io::flush_out();
    if let Some(cmd) = state.cmds.accept_command() {
      if let Err(x) = run_cmd(&mut state, cmd) {
        println!("{}", x);
        break;
      }
    } else {
      println!("Unknown command")
    }
  }
  return;
}
