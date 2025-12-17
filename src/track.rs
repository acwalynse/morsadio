use std::collections::HashMap;
use std::{thread, time::Duration};
use actually_beep::beep_with_hz_and_millis;

/// Parses a plain text with morse code to a playable track.
/// Returns a vector of tuples that can be used in `track::play(Vec<(u32, u32)>)` or an error
/// string.
pub fn parse(track: &str, hz: u32, short_ms: u32, long_ms: u32) -> Result<Vec<(u32, u32)>, String> {
    let lmap: HashMap<char, (u32, u32)> = HashMap::from([
        ('.', (hz, short_ms)),
        ('-', (hz, long_ms )),
        ('_', (hz, long_ms )),
        (' ', (0 , short_ms)),
        ('/', (0 , long_ms )),
    ]);

    let mut parsed: Vec<(u32, u32)> = Vec::new();
    for c in track.chars() { 
        match lmap.get(&c) {
            Some(t) => parsed.push(*t),
            None    => return Err(String::from("Track contains non-morse symbols"))
        }
    }

    Ok(parsed)
}

/// Plays a sound from vector of tuples.
/// Tuple should contain hertz as a first item and millis as a second one.
pub fn play(track: Vec<(u32, u32)>) {
    for unit in track.iter() {
        if unit.0 == 0 {
            thread::sleep(Duration::from_millis(unit.1 as u64));
            continue;
        }
        beep_with_hz_and_millis(unit.0, unit.1).expect("Cannot play audio");
    } 
}
