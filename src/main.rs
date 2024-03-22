use clap::{command, Arg, ArgAction};
use std::fs;
fn string_from_u8(data: &[u8]) -> String {
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(data, true);
    let encoding = detector.guess(None, true);
    encoding.decode(data).0.to_string()
}

fn exec(path: Vec<String>, ext: &Vec<String>) {
    for i in path {
        if fs::metadata(&i).unwrap().is_dir() {
            exec(
                fs::read_dir(&i)
                    .unwrap()
                    .filter(|v| v.is_ok())
                    .map(|v| v.unwrap().path().to_str().unwrap().to_string())
                    .collect(),
                ext,
            );
            continue;
        }
        if ext.len() > 0 {
            if let Some(e) = std::path::PathBuf::from(&i).extension() {
                if ext
                    .iter()
                    .find(|v| &&e.to_string_lossy().to_string() == v)
                    .is_none()
                {
                    continue;
                }
            }
        }
        if let Ok(data) = fs::read(&i) {
            let data = string_from_u8(&data);
            if fs::write(&i, data.as_bytes()).is_ok() {
                println!("Converted {} to utf-8!", i)
            }
        } else {
            println!("cannot read {}!", i)
        }
    }
}

fn main() {
    let matches = command!()
        .arg(Arg::new("path").required(true).action(ArgAction::Append))
        .arg(
            Arg::new("ext")
                .short('e')
                .long("ext")
                .help("Filename extension filter. etc.: txt,lrc"),
        )
        .get_matches();
    let paths = matches
        .get_many::<String>("path")
        .unwrap_or_default()
        .map(|v| v.as_str().to_string())
        .collect::<Vec<_>>();
    let ext = matches
        .get_one::<String>("ext")
        .cloned()
        .unwrap_or(String::new())
        .split(',')
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .collect();
    exec(paths, &ext)
}
