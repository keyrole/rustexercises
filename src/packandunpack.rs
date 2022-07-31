use std::fs::File;
use flate2::{write::GzEncoder, Compression};
use flate2::{read::GzDecoder};
use std::path::Path;
use tar::Archive;
use std::path::PathBuf;


pub fn pack(src_path: &str, dst_path: &str, path_in_archive: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tar_gz = File::create(dst_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(path_in_archive, Path::new(src_path))?;
    Ok(())
}

pub fn unpack(file_path: &str, unpack_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tar_gz = File::open(file_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(unpack_to)?;
    Ok(())
}

pub fn unpack_and_strip_prefix(file_path: &str, unpack_to: &str, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut archive = Archive::new(GzDecoder::new(file));
    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, Box<dyn std::error::Error>> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
    Ok(())
}