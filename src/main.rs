extern crate clap;
mod repl;

use repl::{REPLHelper, get_config};


use rustyline::error::ReadlineError;
use rustyline::{Editor};

use clap::{App, crate_version};

fn main()->rustyline::Result<()> {
    env_logger::init();

    let _matches = App::new("mini-sqlite")
                          .version("0.0.1")
                          .author("speed1313")
                          .about("mini-sqlite for study")
                          .get_matches();

    let config = get_config();

    let helper = REPLHelper::new();

    let mut repl = Editor::with_config(config);
    repl.set_helper(Some(helper));

    if repl.load_history("history").is_err(){
        println!("No previous history.");
    }

    let mut count = 1;
    loop{
        if count == 1 {
            println!("{}{}{}{}{}",
                format!("mini-sqlite-{}\n",crate_version!()),
                "Enter .exit to quit.\n",
                "Enter .help for usage hints.\n",
                "Connected to a transient in-memory database.\n",
                "Use '.open FILENAME' to reopen on a persistent database.");
        }

        let p = format!("mini-sqlite | {}> ", count);
        repl.helper_mut()
            .expect("No helper found")
            .colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p );

        let readline = repl.readline(&p);
        match readline {
            Ok(command)=>{
                repl.add_history_entry(command.as_str());
                if command.eq(".exit"){
                    break;
                }else{
                    println!("Error: unknown command or invalid arguments: '{}'. Enter '.help'",&command);
                }
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof)=>{
                break;
            },
            Err(err)=>{
                println!("Error: {:?}",err);
                break;
            }
        }
        count += 1;
    }
    repl.save_history("history").unwrap();

    Ok(())
}
