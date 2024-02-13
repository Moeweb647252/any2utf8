fn string_from_u8(data: &[u8]) -> String {
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(data, true);
    let encoding = detector.guess(None, true);
    encoding.decode(data).0.to_string()
}

fn main() {
    let mut args = std::env::args();
    args.next();
    for i in args {
        if let Ok(data) = std::fs::read(&i) {
            let data = string_from_u8(&data);
            if std::fs::write(&i, data.as_bytes()).is_ok() {
                println!("Converted {} to utf-8!", i)
            }
        } else {
            println!("cannot read {}!", i)
        }
    }
}
