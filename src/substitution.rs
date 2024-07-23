use crate::context::Context;

use std::fs::File;
use std::env;
use std::io::Read;
use std::{collections::HashMap, vec::Vec};
use regex::{Captures, Regex, Replacer};

pub trait Substitutor {
    fn substitute(context: &Context, string: String) -> String;
}

pub struct VariableSubstitutor;

impl Substitutor for VariableSubstitutor {
    fn substitute(context: &Context, string: String) -> String {
        let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
        re.replace_all(string.as_str(), |caps: &Captures| { context.variables.get(&caps[1]).unwrap() }).to_string()
    }
}

pub struct EnvSubstitutor;

impl Substitutor for EnvSubstitutor {
    fn substitute(context: &Context, string: String) -> String {
        let re = Regex::new(r"\{\{env:(\w+)\}\}").unwrap();
        re.replace_all(string.as_str(), |caps: &Captures| { env::var(&caps[1]).unwrap() }).to_string()
    }
}

pub struct FileSubstitutor;

impl Substitutor for FileSubstitutor {
    fn substitute(context: &Context, string: String) -> String {
        let re = Regex::new(r"\{\{file:(\w+)\}\}").unwrap();
        re.replace_all(string.as_str(), |caps: &Captures|
        {
            let mut file = File::open(context.files.get(&caps[1]).unwrap()).unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            buf
        }).to_string()
    }
}