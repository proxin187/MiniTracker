use std::env;
use std::process;

mod ast;
mod lexer;
mod player;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Help: MiniTracker [FILENAME]");
        process::exit(1);
    }

    let tokens = lexer::lexer(&args[1])?;
    println!("Tokens: {:?}", tokens);
    let patterns = ast::make(&tokens)?;
    println!("Patterns: {:?}", patterns);
    player::play_beat(patterns)?;
    return Ok(());
}


