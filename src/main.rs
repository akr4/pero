extern crate clap;
extern crate colored;
extern crate walkdir;

use std::collections::HashMap;
use std::path::Path;

use project::Project;

mod cargo;
mod dir_size;
mod display;
mod gradle;
mod maven;
mod npm;
mod project;
mod sbt;
mod statistics;

struct ProjectIterator {
    dir_iter: walkdir::IntoIter,
}

impl ProjectIterator {
    pub fn new<P: AsRef<Path>>(path: P) -> ProjectIterator {
        ProjectIterator {
            dir_iter: walkdir::WalkDir::new(path).into_iter(),
        }
    }
}

impl Iterator for ProjectIterator {
    type Item = Box<Project>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            match self.dir_iter.next() {
                Some(entry) => {
                    let entry = entry.ok()?;
                    if let Some(project) = detect_project(entry.path()) {
                        return Some(project);
                    }
                }
                _ => return None,
            }
        }
    }
}

type ProjectDetector = fn(path: &Path) -> Option<Box<Project>>;

fn detect_project(path: &Path) -> Option<Box<Project>> {
    let detectors: Vec<ProjectDetector> = vec![
        maven::detect,
        gradle::detect,
        cargo::detect,
        sbt::detect,
        npm::detect,
    ];
    detectors.iter().flat_map(|x| x(path)).next()
}

enum ColorOption {
    ALWAYS,
    NEVER,
}

impl ColorOption {
    fn from(s: &str) -> ColorOption {
        match s {
            "always" => ColorOption::ALWAYS,
            _ => ColorOption::NEVER,
        }
    }
}

fn main() {
    let matches = clap::App::new("pero")
        .version(option_env!("COMMIT_HASH").unwrap_or(""))
        .arg(clap::Arg::with_name("DIR").required(true))
        .arg(
            clap::Arg::with_name("color")
                .short("c")
                .long("color")
                .help("Control output color: always (default), never")
                .takes_value(true),
        )
        .get_matches();
    let dir = matches.value_of("DIR").unwrap();

    let color_option = ColorOption::from(matches.value_of("color").unwrap_or("always"));
    if let ColorOption::NEVER = color_option {
        colored::control::SHOULD_COLORIZE.set_override(false)
    }

    let mut projects_map = HashMap::new();
    for p in ProjectIterator::new(dir) {
        display::print_project(&*p);

        let type_name = p.type_name().to_string();
        let mut projects = projects_map.entry(type_name).or_insert_with(Vec::new);
        projects.push(p);
    }
    let stats: HashMap<String, statistics::Statistics> = projects_map
        .iter()
        .map(|(project_type, projects)| {
            (project_type.clone(), statistics::calc_statistics(projects))
        })
        .collect();

    display::print_statistics(&stats);
}
