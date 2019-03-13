extern crate rustyline;
extern crate logos;

use rustyline::error::ReadlineError;
use rustyline::Editor;

const HISTORY_FILENAME: &str = ".jojolisp-history";

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(HISTORY_FILENAME).is_err() {
        println!("No previous history.");
    }
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Rustyline error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(HISTORY_FILENAME).unwrap();
}
