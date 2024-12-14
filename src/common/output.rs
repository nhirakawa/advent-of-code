use std::path::PathBuf;
use std::process::Command;
use std::{fs::File, io::Write, path::Path};

use log::warn;

use super::base::{Day, Year};

#[allow(dead_code)]
pub fn write_dot(path: &str, dot: &str) {
    let output_directory = Path::new("../output");

    let dot_path = Path::new(path);
    let dot_path = output_directory.join(dot_path);

    let png_path = dot_path.with_extension("png");

    let was_written = write_internal(dot_path.as_path(), dot.as_bytes());
    if !was_written {
        return;
    }

    if let Err(process_output) = Command::new("neato")
        .arg("-Tpng")
        .arg(dot_path.as_path())
        .arg("-o")
        .arg(png_path.as_path())
        .output()
    {
        warn!("Could not run neato for path {} - {}", path, process_output);
    }
}

#[allow(dead_code)]
pub fn write_output(path: &Path, out: &str) -> bool {
    write_internal(path, out.as_bytes())
}

#[allow(dead_code)]
pub fn write_bytes(path: &Path, out: &[u8]) -> bool {
    write_internal(path, out)
}

#[allow(dead_code)]
pub fn get_output_directory(year: Year, day: Day) -> PathBuf {
    let output_directory = Path::new("output");
    let year_directory = output_directory.join(year.to_string());

    year_directory.join(day.to_string())
}

#[allow(dead_code)]
pub fn writer(path: &Path) -> impl Write {
    let directory = path.parent().unwrap();
    if let Err(e) = std::fs::create_dir_all(directory) {
        warn!("Could not create output directory - {}", e);
    }

    File::create(path).unwrap()
}

// todo(@nhirakawa) - this is horrendously un-idiomatic - fix this
fn write_internal(path: &Path, out: &[u8]) -> bool {
    let directory = path.parent().unwrap();
    if let Err(e) = std::fs::create_dir_all(directory) {
        warn!("Could not create output directory - {}", e);
        return false;
    }

    if let Ok(mut file) = File::create(path) {
        file.write_all(out).is_ok()
    } else {
        warn!("Could not create file {:?}", path);
        false
    }
}
