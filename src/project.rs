use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

pub trait Project {
    fn path(&self) -> &Path;
    fn type_name(&self) -> &str;
    fn size(&self) -> Result<u64, Error>;
}
