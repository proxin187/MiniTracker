use rodio::{Decoder, OutputStream, source::Source, Sink};
use crate::ast::{Patterns, Ast};

use std::{thread, time::Duration};
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

fn play_audio(sound: &(String, f32), sounds: &HashMap<String, String>, stream_handle: &rodio::OutputStreamHandle) {
    let path = sounds.get(&sound.0).ok_or::<Box<dyn std::error::Error>>("failed to find main pattern".into()).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.set_speed(sound.1);
    sink.detach();
}

pub fn play_beat(patterns: Patterns) -> Result<(), Box<dyn std::error::Error>> {
    let (stream, stream_handle) = OutputStream::try_default()?;
    let main = patterns.get("main").ok_or::<Box<dyn std::error::Error>>("failed to find main pattern".into())?;
    let mut bpm = 0.0;

    let mut sounds: HashMap<String, String> = HashMap::new();

    let mut queue: Vec<(String, f32)> = Vec::new();

    for instruction in main {
        match instruction {
            Ast::Bpm(new_bpm) => {
                bpm = *new_bpm;
            },
            Ast::Load(name, path) => {
                sounds.insert(name.to_string(), path.to_string());
            },
            Ast::Play(name, speed) => {
                queue.push((name.to_string(), *speed));
            },
            Ast::Newline => {
                println!("BEAT");
                for sound in &queue {
                    play_audio(sound, &sounds, &stream_handle);
                }
                queue = Vec::new();
                thread::sleep(Duration::from_secs_f32((bpm / 60 as f32) / 8 as f32));
            },
            _ => {},
        }
    }

    return Ok(());
}


