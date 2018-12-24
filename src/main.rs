extern crate clap;
extern crate colored;
extern crate walkdir;

use std::collections::HashMap;
use std::path::Path;

use project::Project;

mod cargo;
mod dir_size;
mod gradle;
mod maven;
mod npm;
mod project;
mod sbt;

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

fn format_number(size: u64) -> String {
    let mut buf = String::new();
    for (c, i) in size.to_string().as_str().chars().rev().zip(0..) {
        if i != 0 && i % 3 == 0 {
            buf.push(',');
        }
        buf.push(c);
    }
    buf.as_str().chars().rev().collect()
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

type Stats = (u64, u64);

fn calc_stats<'a, ITER>(projects: ITER) -> Stats
where
    ITER: Iterator<Item = &'a Box<Project>>,
{
    projects.fold((0, 0), |(count, size), p| {
        (count + 1, size + p.size().ok().unwrap_or(0))
    })
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
        print_project(&*p);

        let type_name = p.type_name().to_string();
        let mut projects = projects_map.entry(type_name).or_insert_with(Vec::new);
        projects.push(p);
    }
    let stats: HashMap<String, Stats> = projects_map
        .iter()
        .map(|(project_type, projects)| {
            let it = projects.iter();
            (project_type.clone(), calc_stats(it))
        })
        .collect();

    print_statistics(&stats);
}

fn print_project(project: &Project) {
    use colored::*;
    println!(
        "{}: {} ({})",
        project.path().display(),
        project.type_name().blue(),
        project
            .size()
            .map(format_number)
            .unwrap_or_else(|_| "-".to_string())
            .bright_white()
    );
}

fn print_statistics(stats: &HashMap<String, Stats>) {
    use colored::*;
    println!();
    println!("{}", "Statistics".green().bold());
    println!("{}", "==========".white());
    for (project_type, (count, size)) in stats {
        println!(
            "{}: {} projects, {} bytes",
            project_type.blue(),
            format_number(*count).bright_white(),
            format_number(*size).bright_white()
        );
    }
    let total_stats: Stats = stats
        .iter()
        .fold((0, 0), |acc, x| (acc.0 + (x.1).0, acc.1 + (x.1).1));
    println!("{}", "----------".white());
    println!(
        "{}: {} projects, {} bytes",
        "Total".blue(),
        format_number(total_stats.0).bright_white(),
        format_number(total_stats.1).bright_white()
    );
}
