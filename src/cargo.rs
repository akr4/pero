use dir_size;
use project;
use std::fs::read_dir;
use std::path::Path;

pub fn detect(path: &Path) -> Option<Box<project::Project>> {
    match path.file_name()?.to_str() {
        Some("Cargo.toml") => path
            .parent()
            .map(|p| Box::new(Cargo::new(p)) as Box<project::Project>),
        _ => None,
    }
}

struct Cargo {
    path: Box<Path>,
}

impl Cargo {
    fn new(path: &Path) -> Cargo {
        Cargo {
            path: Box::from(path),
        }
    }
}

impl project::Project for Cargo {
    fn path(&self) -> &Path {
        &self.path
    }

    fn type_name(&self) -> &str {
        "Cargo"
    }

    fn size(&self) -> Result<u64, project::Error> {
        match find_build_dir(&self.path).map_err(project::Error::Io)? {
            Some(path) => dir_size::calc_size_recursively(&path).map_err(project::Error::Io),
            _ => Ok(0),
        }
    }
}

fn find_build_dir(project_root: &Path) -> Result<Option<Box<Path>>, std::io::Error> {
    for entry in read_dir(project_root)? {
        let entry = entry?;
        if entry.file_type()?.is_dir()
            && entry
                .file_name()
                .to_str()
                .map(|x| x == "target")
                .unwrap_or(false)
        {
            return Ok(Some(entry.path().into_boxed_path()));
        }
    }
    Ok(None)
}
