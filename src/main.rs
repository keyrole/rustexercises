mod packandunpack;
use packandunpack::pack;
use packandunpack::unpack;

use crate::packandunpack::unpack_and_strip_prefix;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // pack("/tmp/rust/log", "/tmp/rust/log.tar.gz", "logs").unwrap();
    // unpack("/tmp/rust/log.tar.gz", "/tmp");
    unpack_and_strip_prefix("/tmp/rust/log.tar.gz", "/tmp", "logs")?;
    println!("success");
    Ok(())
}