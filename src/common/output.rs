use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::{fs::File, io::Write, path::Path};

use anyhow::anyhow;
use anyhow::bail;
use log::{info, warn};

use super::base::{Day, Year};

#[allow(dead_code)]
pub struct DotConfig {
    pub output_format: OutputFormat,
    pub layout_engine: LayoutEngine,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OutputFormat {
    Png,
    Svg,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LayoutEngine {
    Dot,
    Neato,
    Fdp,
    Osage,
}

impl LayoutEngine {
    pub fn as_command(&self) -> &str {
        match self {
            LayoutEngine::Dot => "dot",
            LayoutEngine::Neato => "neato",
            LayoutEngine::Fdp => "fdp",
            LayoutEngine::Osage => "osage",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct OutputWriter {
    year: Year,
    day: Day,
}

impl OutputWriter {
    pub fn new(year: Year, day: Day) -> Self {
        Self { year, day }
    }

    pub fn output_directory(&self) -> PathBuf {
        Path::new("output")
            .join(self.year.to_string())
            .join(self.day.to_string())
    }

    pub fn write_dot(&self, filename: &str, dot: &str, config: DotConfig) -> anyhow::Result<()> {
        if filename.contains(".") {
            bail!("Filename should not contain a file extension");
        }

        let file_extension = match config.output_format {
            OutputFormat::Png => "png",
            OutputFormat::Svg => "svg",
        };

        let dot_path = self.output_directory().join(filename.to_owned() + ".dot");

        self.write_internal(&dot_path, dot.as_bytes())?;

        let file_path = self
            .output_directory()
            .join(filename.to_owned() + "." + file_extension);

        let program_name = config.layout_engine.as_command();

        if let Err(process_output) = Command::new(program_name)
            .arg(format!("-T{file_extension}"))
            .arg(&dot_path)
            .arg("-o")
            .arg(&file_path)
            .output()
        {
            bail!("Could not run {program_name} for path {dot_path:?} - {process_output}",);
        } else {
            info!("Graph written to {file_path:?}");
        }

        Ok(())
    }

    fn write_internal(&self, path: &Path, out: &[u8]) -> anyhow::Result<()> {
        let directory = path
            .parent()
            .ok_or(anyhow!("Could not get parent directory"))?;

        std::fs::create_dir_all(directory).or_else(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                Ok(())
            } else {
                Err(e)
            }
        })?;

        fs::write(path, out).map_err(Into::into)
    }
}

#[allow(dead_code)]
pub fn write_output(path: &Path, out: &str) -> bool {
    write_internal(path, out.as_bytes())
}

#[allow(dead_code)]
pub fn get_output_directory(year: Year, day: Day) -> PathBuf {
    let output_directory = Path::new("output");
    let year_directory = output_directory.join(year.to_string());

    year_directory.join(day.to_string())
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
