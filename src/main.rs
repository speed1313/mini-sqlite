use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    
    let mut repl = Editor::<()>::new();

    if repl.load_history("history.txt").is_err(){
        println!("No previous history.");
    }
    loop{
        let readline = repl.readline(">> ");

        match readline {
            Ok(line)=>{
                repl.add_history_entry(line.as_str());
                println!("Line: {}",line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof)=>{
                println!("CTRL-D");
                break;
            },
            Err(err)=>{
                println!("Error: {:?}",err);
                break
            }
        }
    }

    repl.save_history("history.txt").unwrap();
}
