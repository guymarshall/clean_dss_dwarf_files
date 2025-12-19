// TODO: Only log current directory e.g. "M42 - Pleiades"
// TODO: Count number of directories, then only log "CURRENT/TOTAL"
// TODO: Parallelise

use glob::glob;
use std::fs;
use std::io;
use std::path::PathBuf;

fn delete_dirs_recursively(root: &str, name: &str) -> io::Result<()> {
    for entry in fs::read_dir(root)? {
        let entry: fs::DirEntry = entry?;
        let path: PathBuf = entry.path();

        if !path.is_dir() {
            continue;
        }

        if let Some(directory_name) = path
            .file_name()
            .and_then(|filename: &std::ffi::OsStr| filename.to_str())
            && directory_name == name
        {
            println!("Removing directory {:?}", path);
            fs::remove_dir_all(&path)?;
            continue;
        }
        delete_dirs_recursively(path.to_str().unwrap(), name)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    const FILES: [&str; 7] = [
        "img_reference.png",
        "img_stacked_all.tif",
        "img_stacked_counter.png",
        "shotsInfo.json",
        "*stacked*",
        "*.info.txt",
        "*.stackinfo.txt",
    ];

    const DIRECTORIES: [&str; 1] = ["Thumbnail"];

    for pattern in &FILES {
        let recursive_pattern: String = format!("**/{}", pattern);

        for entry in glob(&recursive_pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) if path.is_file() => {
                    println!("Removing file {:?}", path);
                    fs::remove_file(path)?;
                }
                Err(e) => eprintln!("Glob error: {}", e),
                _ => {}
            }
        }
    }

    for directory_name in &DIRECTORIES {
        delete_dirs_recursively(".", directory_name)?;
    }

    Ok(())
}
