use dir_size;
use project;
use project::Project;
use std::fs::read_dir;
use std::path::Path;

pub fn detect(path: &Path) -> Option<Box<Project>> {
    if path.file_name()?.to_str()? == "package.json"
        && has_node_modules(&path.parent()?)
        && !is_nested_in_node_modules(path)
    {
        path.parent().map(|p| Box::new(Npm::new(p)) as Box<Project>)
    } else {
        None
    }
}

struct Npm {
    path: Box<Path>,
}

impl Npm {
    fn new(path: &Path) -> Npm {
        Npm {
            path: Box::from(path),
        }
    }
}

impl Project for Npm {
    fn path(&self) -> &Path {
        &self.path
    }

    fn type_name(&self) -> &str {
        "npm"
    }

    fn size(&self) -> Result<u64, project::Error> {
        let p = node_module_path(&self.path).expect("Project should have node_modules");
        dir_size::calc_size_recursively(&p).map_err(project::Error::Io)
    }
}

fn is_nested_in_node_modules(path: &Path) -> bool {
    match path.parent() {
        Some(parent) => match parent.file_name().and_then(|x| x.to_str()) {
            Some(file_name) if file_name == "node_modules" => true,
            _ => is_nested_in_node_modules(parent),
        },
        _ => false,
    }
}

fn has_node_modules(project_root: &Path) -> bool {
    node_module_path(project_root).is_some()
}

fn node_module_path(project_root: &Path) -> Option<Box<Path>> {
    for entry in read_dir(project_root).ok()? {
        let entry = entry.ok()?;
        if entry.path().is_dir() && entry.file_name().to_str()? == "node_modules" {
            return Some(entry.path().into_boxed_path());
        }
    }
    None
}
