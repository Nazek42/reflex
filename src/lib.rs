extern crate regex;

use regex::Regex;
use std::boxed::Box;

fn convert_regex(re: &str) -> String {
    format!("^{}", re)
}

pub struct Lexer<T> {
    rules: Vec<(Regex, Box<Fn(&str) -> Option<T>>)>,
}

impl <T: Copy + 'static> Lexer<T> {
    pub fn new() -> Lexer<T> {
        Lexer::<T> { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, re: &str, rule: Box<Fn(&str)->T>) {
        let func = Box::new(move |tok: &str| Some(rule(tok)));
        self.rules.push((Regex::new(convert_regex(re).as_ref()).unwrap(), func));
    }

    pub fn add_simple(&mut self, re: &str, token: T) {
        let func = Box::new(move |tok: &str| Some(token));
        self.rules.push((Regex::new(convert_regex(re).as_ref()).unwrap(), func));
    }

    pub fn add_noop(&mut self, re: &str) {
        let func = Box::new(|tok: &str| None);
        self.rules.push((Regex::new(convert_regex(re).as_ref()).unwrap(), func));
    }

    pub fn lex(&self, raw: &str) -> Vec<T> {
        let mut tokens = Vec::new();
        let mut rest = String::new();
        let mut text = String::from(raw);
        while !text.is_empty() {
            for &(ref re, ref func) in self.rules.iter() {
                if let Some(mat) = re.find(text.clone().as_ref()) {
                    if let Some(token) = func(&text[mat.start()..mat.end()]) {
                        tokens.push(token);
                    }
                    rest = String::from(&text[mat.end()..]);
                    text = rest.clone();
                }
            }
        }
        tokens
    }
}
