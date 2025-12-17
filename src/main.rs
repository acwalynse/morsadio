use std::process;
use clap::Parser;
use morsadio::track;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Frequency of beeps in hertz
    #[arg(short = 'z', long, default_value_t = 440)]
    hertz: u32,
    
    /// How long should '.' be speaked in milliseconds
    #[arg(short, long, default_value_t = 100)]
    short_beep_ms: u32,
    
    /// How long should '-' be speaked in milliseconds
    #[arg(short, long, default_value_t = 250)]
    long_beep_ms: u32,
    
    /// Morse code as a plain text
    #[arg(trailing_var_arg = true)]
    track: Vec<String>,
}

fn main() {
    let args: Args = Args::parse();
    
    match args.track.first() {
        Some(_) => (),
        None => { eprintln!("Track must be not empty"); process::exit(1); } 
    }
    
    match track::parse(args.track.first().unwrap(), args.hertz, args.short_beep_ms, args.long_beep_ms) {
        Ok(parsed) => track::play(parsed),
        Err(error) => { eprintln!("{}", error); process::exit(1); }
    }
}
