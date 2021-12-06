use std::fs::File;
use std::io::BufReader;

fn get_default_filename(challenge: u8) -> String {
    let file_index = if challenge % 2 == 0 {
        challenge / 2
    } else {
        (challenge / 2) + 1
    };

    format!("raw_data/input{}.txt", file_index)
}

pub fn get_file(args: &[String], challenge: u8) -> Result<BufReader<File>, std::io::Error> {
    let default_filename = get_default_filename(challenge);
    let filename = match args.len() {
        0 => default_filename.as_str(),
        _ => args[0].as_str(),
    };

    match File::open(filename) {
        Ok(f) => Ok(BufReader::new(f)),
        Err(e) => {
            println!("failed to load file: {:?}", e);
            Err(e)
        }
    }
}
