use std::io::Read;
use std::fs::File;
use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    pub variables: HashMap<String, String>,
    pub files: HashMap<String, String>,
}

impl Context {
    pub fn new_from_yaml(file_path: &PathBuf) -> Self {

        let file = File::open(file_path).unwrap();
        from_reader(file).unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_variables() {
        let mut path = PathBuf::new();
        path.push("resource/tests/context.yml");
        let context = Context::new_from_yaml(&path);

        assert_eq!(context.variables.get("FOO").unwrap().clone(), "My cheese".to_string());
        assert_eq!(context.variables.get("BAR").unwrap().clone(), "My hands".to_string());
    }
}