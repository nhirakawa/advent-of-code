use std::process::Command;
use std::{fs::File, io::Write, path::Path};

use log::warn;

pub fn write_dot(path: &str, o: &str) {
    write_output(path, o);

    let rendered_path = path.replace("dot", "png");

    if let Err(process_output) = Command::new("neato")
        .arg("-Tpng")
        .arg(path)
        .arg("-o")
        .arg(rendered_path)
        .output()
    {
        warn!("Could not run neato for path {} - {}", path, process_output);
    }
}

pub fn write_output(path: &str, o: &str) {
    let path = Path::new(path);

    if let Ok(mut file) = File::create(path) {
        file.write_all(o.as_bytes());
    } else {
        warn!("Could not create file {:?}", path);
    }
}
