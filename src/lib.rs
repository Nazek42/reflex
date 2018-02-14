extern crate regex;

use regex::Regex;
use std::boxed::Box;

pub struct Ruleset<T>(Vec<(Regex, Box<Fn(&str) -> Option<T>>)>);

impl <T: Clone + 'static> Ruleset<T> {
    pub fn new() -> Ruleset<T> {
        Ruleset::<T>(Vec::new())
    }

    pub fn add_rule<F: 'static + Fn(&str)->T>(&mut self, re: &str, rule: F) {
        let func = Box::new(move |_tok: &str| Some(rule(_tok)));
        self.0.push((Regex::new(convert_regex(re).as_ref()).unwrap(), func));
    }

    pub fn add_simple(&mut self, re: &str, token: T) {
        let func = Box::new(move |_tok: &str| Some(token.clone()));
        self.0.push((Regex::new(convert_regex(re).as_ref()).unwrap(), func));
    }

    pub fn add_noop(&mut self, re: &str) {
        let func = Box::new(|_tok: &str| None);
        self.0.push((Regex::new(convert_regex(re).as_ref()).unwrap(), func));
    }
}

pub struct Lexer<'a, T: Clone + 'static> {
    rules: &'a Ruleset<T>,
    text: String,
}

impl <'a, T: Clone + 'static> Iterator for Lexer<'a, T> {
    type Item = Result<T, String>;
    fn next(&mut self) -> Option<Result<T, String>> {
        let mut result: Option<Result<T, String>> = None;
        let mut matched;
        while result.is_none() {
            matched = false;
            for &(ref re, ref func) in self.rules.0.iter() {
                if self.text.is_empty() {
                    return None;
                }
                if let Some(mat) = re.find(self.text.clone().as_ref()) {
                    if let Some(token) = func(&self.text[mat.start()..mat.end()]) {
                        result = Some(Ok(token));
                    }
                    let rest = String::from(&self.text[mat.end()..]);
                    self.text = rest.clone();
                    matched = true;
                    break;
                }
            }
            if !matched {
                result = Some(Err(format!("No rule matched \"{}\"", self.text)));
            }
        }
        result
    }
}

pub fn lex<T: Clone + 'static, S: Into<String>>(rules: &Ruleset<T>, text: S) -> Lexer<T> {
    Lexer {
        rules: rules,
        text: text.into(),
    }
}

fn convert_regex(re: &str) -> String {
    format!("^{}", re)
}
