use std::{fs::File, io::{Read, BufReader}};
use error_chain::error_chain;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Decode(data_encoding::DecodeError);
    }
}

fn main() -> Result<()> {
    let input = File::open("/tmp/rust/log/log1")?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;
    println!("{}", HEXUPPER.encode(digest.as_ref()));
    Ok(())
}

// fn new_file(path: &str) -> Result<File> {
//     let mut file = File::create(path)?;
//     write!(file, "this is a test text")?;
//     Ok(file)
// }

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(context.finish())
}