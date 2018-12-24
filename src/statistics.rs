#[cfg(test)]
use project;
use project::Project;
#[cfg(test)]
use std::path::Path;

pub type Statistics = (u64, u64);

pub fn calc_statistics<'a, ITER>(projects: ITER) -> Statistics
where
    ITER: IntoIterator<Item = &'a Box<Project>>,
{
    projects.into_iter().fold((0, 0), |(count, size), p| {
        (count + 1, size + p.size().ok().unwrap_or(0))
    })
}

#[cfg(test)]
struct TestingProject;

#[cfg(test)]
impl Project for TestingProject {
    fn path(&self) -> &Path {
        Path::new("/path")
    }

    fn type_name(&self) -> &str {
        "test"
    }

    fn size(&self) -> Result<u64, project::Error> {
        Ok(1)
    }
}

#[test]
fn test_calc_statistics() {
    let p1: Box<Project> = Box::new(TestingProject {});
    let stats = calc_statistics(vec![&p1]);
    assert_eq!(stats.0, 1);
    assert_eq!(stats.1, p1.size().unwrap());
}
