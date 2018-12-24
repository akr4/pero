use project::Project;
use statistics;
use std::collections::HashMap;

pub fn print_project(project: &Project) {
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

pub fn print_statistics(stats: &HashMap<String, statistics::Statistics>) {
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
    let total_stats: statistics::Statistics = stats
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

#[test]
fn test_format_number() {
    assert_eq!(format_number(9), "9");
    assert_eq!(format_number(99), "99");
    assert_eq!(format_number(999), "999");
    assert_eq!(format_number(9_999), "9,999");
    assert_eq!(format_number(99_999), "99,999");
    assert_eq!(format_number(999_999), "999,999");
    assert_eq!(format_number(9_999_999), "9,999,999");
}
