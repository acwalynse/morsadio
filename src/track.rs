use std::{thread, time::Duration};
use actually_beep::beep_with_hz_and_millis;
use regex::Regex;

pub fn parse(track: &str, hz: u32, short_ms: u32, long_ms: u32) -> Result<Vec<(u32, u32)>, String> {
    let mut parsed: Vec<(u32, u32)> = Vec::new();
    
    let reg: Regex = Regex::new(r"[^\.\-_\s/]").unwrap();
    if reg.is_match(track) { return Err(String::from("Track contains non-morse symbols")) }
    
    for unit in track.chars() {
        match unit {
            '.'         => parsed.push((hz, short_ms)),
            '-' | '_'   => parsed.push((hz, long_ms )),
            ' '         => parsed.push((0,  short_ms)),
            '/'         => parsed.push((0,  long_ms )),
            _           => continue
        }    
    }
    
    Ok(parsed)
}

pub fn play(track: Vec<(u32, u32)>) {
    for unit in track.iter() {
        if unit.0 == 0 {
            thread::sleep(Duration::from_millis(unit.1 as u64));
            continue;
        }
        beep_with_hz_and_millis(unit.0, unit.1).unwrap();
    } 
}
