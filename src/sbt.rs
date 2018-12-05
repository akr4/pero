use dir_size;
use project;
use std::fs::read_dir;
use std::path::Path;

pub fn detect(path: &Path) -> Option<Box<project::Project>> {
    match path.file_name()?.to_str() {
        Some("build.sbt") => path
            .parent()
            .map(|p| Box::new(Sbt::new(p)) as Box<project::Project>),
        _ => None,
    }
}

struct Sbt {
    path: Box<Path>,
}

impl Sbt {
    fn new(path: &Path) -> Sbt {
        Sbt {
            path: Box::from(path),
        }
    }
}

impl project::Project for Sbt {
    fn path(&self) -> &Path {
        &self.path
    }

    fn type_name(&self) -> &str {
        "sbt"
    }

    fn size(&self) -> Result<u64, project::Error> {
        let mut size: u64 = 0;
        for dir in find_build_dirs(&self.path).map_err(project::Error::Io)? {
            size += dir_size::calc_size_recursively(&dir).map_err(project::Error::Io)?
        }
        Ok(size)
    }
}

fn find_build_dirs(project_root: &Path) -> Result<Vec<Box<Path>>, std::io::Error> {
    let lib_managed_dir = find_lib_managed_dir(project_root)?;
    let mut dirs = find_target_dirs(project_root)?;
    if lib_managed_dir.is_some() {
        dirs.push(lib_managed_dir.unwrap());
    }
    Ok(dirs)
}

fn find_lib_managed_dir(project_root: &Path) -> Result<Option<Box<Path>>, std::io::Error> {
    for entry in read_dir(project_root)? {
        let entry = entry?;
        if entry.file_type()?.is_dir()
            && entry
                .file_name()
                .to_str()
                .map(|x| x == "lib_managed")
                .unwrap_or(false)
        {
            return Ok(Some(entry.path().into_boxed_path()));
        }
    }

    Ok(None)
}

fn find_target_dirs(project_root: &Path) -> Result<Vec<Box<Path>>, std::io::Error> {
    let mut paths = Vec::new();
    for entry in read_dir(project_root)? {
        let entry = entry?;
        if entry.file_type()?.is_dir()
            && entry
                .file_name()
                .to_str()
                .map(|x| x == "target")
                .unwrap_or(false)
        {
            paths.push(entry.path().into_boxed_path());
        }
    }
    Ok(paths)
}
