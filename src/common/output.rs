use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::anyhow;
use anyhow::bail;
use log::info;

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

    pub fn write_dot<B: AsRef<[u8]>>(
        &self,
        filename: &str,
        dot: B,
        config: DotConfig,
    ) -> anyhow::Result<()> {
        if filename.contains(".") {
            bail!("Filename should not contain a file extension");
        }

        let file_extension = match config.output_format {
            OutputFormat::Png => "png",
            OutputFormat::Svg => "svg",
        };

        let dot_path = self.output_directory().join(filename.to_owned() + ".dot");

        self.write_internal(&dot_path, dot)?;

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

    pub fn write<B: AsRef<[u8]>>(&self, filename: &str, bytes: B) -> anyhow::Result<()> {
        let file_path = self.output_directory().join(filename);
        self.write_internal(&file_path, bytes)
    }

    fn write_internal<B: AsRef<[u8]>>(&self, path: &Path, bytes: B) -> anyhow::Result<()> {
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

        fs::write(path, bytes).map_err(Into::into)
    }
}
