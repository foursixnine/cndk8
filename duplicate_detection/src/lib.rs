//use std::fs::{self, DirEntry, ReadDir};
use std::fs;
use std::path::Path;
use std::io;
use std::io::{Error,Write};
pub fn get_files() -> Result<Vec<File>, Error> {
    //#let files;
    let dir; 
    let mut file_list = vec![];
    dir = fs::read_dir(".").unwrap();
    for entries in dir {
        if let Ok(entries) = entries {
            file_list.push(File { name: entries.path().display().to_string() });
        }
    }

    Ok(file_list)
}

pub struct File {
    name: String,
}


pub fn print() -> io::Result<()> {

let mut stdout = io::stdout();

    let mut file_list = vec![];
    file_list = get_files().unwrap();
    for file in file_list {
        stdout.write(file.name.as_bytes())?;
        stdout.write(b"\n")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Default)]
    struct MockFile {
        set_len_called: Option<u64>,
    }

    #[test]
    fn test_get_files() {
        super::get_files();
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
