use std::collections::HashSet;
use std::path::Path;
use walkdir;

type INODE = (u64, u64);

pub fn calc_size_recursively<P: AsRef<Path>>(path: P) -> Result<u64, std::io::Error> {
    let mut visited_inodes = HashSet::new();
    let mut size: u64 = 0;
    for entry in walkdir::WalkDir::new(path) {
        let entry = entry?;
        if let Some(inode) = get_inode(&entry) {
            if visited_inodes.contains(&inode) {
                // duplicated inode (hard link) found
                continue;
            } else {
                visited_inodes.insert(inode);
            }
        }
        let path = entry.path();
        if path.is_file() {
            size += calc_size(&entry)?;
        }
    }
    Ok(size)
}

#[cfg(not(unix))]
fn calc_size(entry: &walkdir::DirEntry) -> Result<u64, std::io::Error> {
    entry.metadata()?.len()
}

#[cfg(unix)]
fn calc_size(entry: &walkdir::DirEntry) -> Result<u64, std::io::Error> {
    use std::os::unix::fs::MetadataExt;
    // blocks() returns number of blocks in 512-byte units not in actual block size.
    Ok(entry.metadata()?.blocks() * 512)
}

#[cfg(not(unix))]
fn get_inode(entry: &walkdir::DirEntry) -> Option<INODE> {
    None
}

#[cfg(unix)]
fn get_inode(entry: &walkdir::DirEntry) -> Option<INODE> {
    use std::os::unix::fs::MetadataExt;
    let md = entry.metadata().ok()?;
    Some((md.ino(), md.dev()))
}
