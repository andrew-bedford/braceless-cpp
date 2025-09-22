use std::fs;
use std::path::{Path, PathBuf};

use crate::parser::BracelessParser;

pub struct FileProcessor {
    parser: BracelessParser,
    temp_files: Vec<PathBuf>,
}

impl FileProcessor {
    pub fn new() -> Self {
        Self {
            parser: BracelessParser::new(),
            temp_files: Vec::new(),
        }
    }

    pub fn process_args(
        &mut self,
        args: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut modified_args = args.clone();

        for i in 0..args.len() {
            let arg = &args[i];

            if let Some(path) = self.extract_source_file(arg) {
                if self.is_bpp_file(&path) {
                    let temp_file = self.process_bpp_file(&path)?;
                    modified_args[i] = temp_file.to_string_lossy().to_string();
                    self.temp_files.push(temp_file);
                }
            }
        }

        Ok(modified_args)
    }

    pub fn cleanup(&self) {
        for temp_file in &self.temp_files {
            if temp_file.exists() {
                let _ = fs::remove_file(temp_file);
            }
        }
    }

    fn extract_source_file(&self, arg: &str) -> Option<PathBuf> {
        if arg.starts_with('-') {
            return None;
        }

        let path = Path::new(arg);
        if path.exists() && path.is_file() {
            Some(path.to_path_buf())
        } else {
            None
        }
    }

    fn is_bpp_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "bpp")
            .unwrap_or(false)
    }

    fn process_bpp_file(&self, path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

        let processed_content = self
            .parser
            .process(&content)
            .map_err(|e| format!("Failed to process file {}: {}", path.display(), e))?;

        let temp_path = self.create_temp_path(path);
        fs::write(&temp_path, processed_content).map_err(|e| {
            format!(
                "Failed to write temporary file {}: {}",
                temp_path.display(),
                e
            )
        })?;

        Ok(temp_path)
    }

    fn create_temp_path(&self, original_path: &Path) -> PathBuf {
        let parent = original_path.parent().unwrap_or(Path::new("."));
        let stem = original_path.file_stem().unwrap().to_string_lossy();

        // While clang++ doesn't appear care about the file extension being used, the linker does and
        // rejects object files created from source files with unknown extensions. So we use .cpp here.
        let temp_filename = format!(".{}.cpp", stem);

        parent.join(temp_filename)
    }
}
