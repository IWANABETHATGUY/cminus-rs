// mod lexer;
// use lexer::lex::Lexer;
// use lexer::token::{Token, TokenType};

// use std::fs::{read_to_string, File};
// use std::io::Error;
// use std::path;
// fn main() -> Result<(), Error> {
//     let path = path::Path::new("test.txt");
//     let a = read_to_string(path)?;
//     let mut lex = Lexer::new(&a);
//     let list = lex.lex();
//     for token in list {
//         println!("{:?}", token);
//     }
//     Ok(())
// }
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct TV;

impl TV {
    fn new() -> TV {
        TV
    }

    fn on(&self) {
        println!("tv is on, watch movies",);
    }

    fn off(&self) {
        println!("tv is off",);
    }
}

trait Command {
    fn excute(&self);
}

struct TvOnCommand {
    tv: TV,
}
impl TvOnCommand {
    fn new(tv: TV) -> TvOnCommand {
        TvOnCommand { tv }
    }
}
impl Command for TvOnCommand {
    fn excute(&self) {
        self.tv.on();
    }
}

struct TvOffCommand {
    tv: TV,
}
impl TvOffCommand {
    fn new(tv: TV) -> TvOffCommand {
        TvOffCommand { tv }
    }
}
impl Command for TvOffCommand {
    fn excute(&self) {
        self.tv.off();
    }
}

struct RemoteTV {
    commands: HashMap<i32, Box<dyn Command>>,
}
impl RemoteTV {
    fn new() -> RemoteTV {
        RemoteTV {
            commands: HashMap::new(),
        }
    }

    fn insert_commands(&mut self, idx: i32, command: Box<dyn Command>) {
        self.commands.insert(idx, command);
    }

    fn press(&self, idx: i32) {
        if let Some(command) = self.commands.get(&idx) {
            command.excute();
        } else {
            println!("do nothing",);
        }
    }
}
fn main() {
    let tv = TV::new();
    let mut remote_tv = RemoteTV::new();
    remote_tv.press(0);

    remote_tv.insert_commands(1, Box::new(TvOffCommand::new(tv)));
    remote_tv.insert_commands(2, Box::new(TvOnCommand::new(tv)));

    remote_tv.press(1);
    remote_tv.press(2);
}
