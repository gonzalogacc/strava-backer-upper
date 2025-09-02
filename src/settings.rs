
use std::collections::HashMap;
use std::env;

#[derive(Default)]
pub struct Settings {
    variables: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            variables: HashMap::new(),
        }
    }

    pub fn load(&mut self, var_name: &str, ) -> Result<(), &'static str>{
        // Try env if not there try GCP secrets (TODO)

        // 1 - Read from .env
        for (key, value) in env::vars() {
            if var_name == key {
                self.variables.insert(key, value);
                return Ok(());
            }
        }

        // 2 - Read from secrets
        Err("Could not find variable in env")
    }

    pub fn get_value(&self, var_name: &str) -> Result<&String, &'static str> {
        match self.variables.get(var_name) {
            Some(value) => Ok(value),
            None => Err("Variable not loaded"),
        }
    }
}


#[cfg(test)]
mod tests{
    use std::fs::File;
    use tempfile::{tempdir};
    use super::*;
    use std::io::{self, Write};

    #[test]
    fn test_find_key() {
        assert_eq!(1,1);
    }

    #[test]
    fn test_() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".temp_dotenv");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "STRAVA_KEY=somekey\nMYNAME=gonza").unwrap();

        dotenv::from_filename(file_path.display().to_string()).ok();

        let mut settings = Settings::new();
        settings.load("MYNAME").expect("Could not find");
        let my_name = settings.get_value("MYNAME").expect("Could not load var MYNAME");

        assert_eq!(my_name, "gonza");
    }
}