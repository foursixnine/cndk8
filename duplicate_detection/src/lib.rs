//use std::fs::{self, DirEntry, ReadDir};
use std::fs;
use std::io;
use std::io::{Error, Write};
pub fn get_files() -> Result<Vec<File>, Error> {
    let mut file_list = vec![];
    let dir = fs::read_dir(".").unwrap();
    for entries in dir {
        file_list.push(File {
            name: entries?.path().display().to_string(),
        });
    }

    Ok(file_list)
}

pub struct File {
    name: String,
}

pub fn print() -> Result<(), Error> {
    let mut stdout = io::stdout();

    for file in get_files().unwrap() {
        stdout.write_all(file.name.as_bytes())?;
        stdout.write_all(b"\n")?;
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
