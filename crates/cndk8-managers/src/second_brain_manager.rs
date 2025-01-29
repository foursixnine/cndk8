use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct SecondBrain;
pub struct SecondBrainManager;

impl SecondBrainManager {
    pub fn append_to_brain(text: &str, format: SecondBrainSupportedFormats) -> io::Result<()> {
        let brain_location = get_brain_location();
        let mut file = OpenOptions::new()
            .append(true)
            .open(brain_location)
            .unwrap();
        file.write_all(text.as_bytes())
            .expect("failed to write/append to brain");
        Ok(())
    }
}

// Add any other necessary functions or types here
fn get_brain_location() -> String {
    match std::env::var("BRAIN_LOCATION") {
        Ok(location) => location,
        Err(_) => panic!("Please set the BRAIN_LOCATION environment variable"),
    }
}

pub enum SecondBrainSupportedFormats {
    Markdown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;
    fn setup_test_environment() {
        unsafe {
            // Set a temporary brain location for testing
            env::set_var("BRAIN_LOCATION", "test_brain.md");
            std::fs::File::create("test_brain.md").unwrap();
        }
    }

    fn cleanup_test_environment() {
        // Remove the test file if it exists
        let _ = std::fs::remove_file("test_brain.md");
    }

    #[test]
    fn test_append_to_brain_basic() {
        setup_test_environment();
        let result =
            SecondBrainManager::append_to_brain("new data", SecondBrainSupportedFormats::Markdown);
        assert!(result.is_ok());

        // Verify the content of the file
        let mut file = File::open("test_brain.md").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "new data");

        cleanup_test_environment();
    }

    #[test]
    fn test_get_brain_location() {
        setup_test_environment();
        let brain_location: String;
        unsafe {
            brain_location = env::var("BRAIN_LOCATION").unwrap();
        }
        let location = get_brain_location();
        assert_eq!(location, brain_location);
        cleanup_test_environment();
    }

    #[test]
    fn test_append_to_brain_no_space() {
        setup_test_environment();
        let result =
            SecondBrainManager::append_to_brain("test", SecondBrainSupportedFormats::Markdown);
        assert!(result.is_err());
        cleanup_test_environment();
    }
}
