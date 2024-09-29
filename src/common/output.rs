use std::process::Command;
use std::{fs::File, io::Write, path::Path};

use log::warn;

pub fn write_dot(path: &str, dot: &str) {
    let output_directory = Path::new("../output");

    let dot_path = Path::new(path);
    let dot_path = output_directory.join(dot_path);

    let png_path = dot_path.with_extension("png");

    let was_written = write_internal(dot_path.as_path(), dot);
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
pub fn write_output(path: &str, out: &str) -> bool {
    let path = Path::new(path);

    return write_internal(path, out);
}

// todo(@nhirakawa) - this is horrendously un-idiomatic - fix this
fn write_internal(path: &Path, out: &str) -> bool {
    let directory = path.parent().unwrap();
    if let Err(e) = std::fs::create_dir_all(directory) {
        warn!("Could not create output directory - {}", e);
        return false;
    }

    if let Ok(mut file) = File::create(path) {
        return file.write_all(out.as_bytes()).is_ok();
    } else {
        warn!("Could not create file {:?}", path);
        return false;
    }
}
