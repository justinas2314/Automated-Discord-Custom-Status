mod parser;
mod get_apps;
mod client;

use fancy_regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;


pub struct Values {
    pub match_text: String,
    pub match_regex: Regex,
    pub text: Option<String>,
    pub emoji: Option<String>,
    pub regex: Option<String>,
    pub regex_compiled: Option<Regex>,
    pub format: Option<String>,
    pub fallback: Option<String>,
    pub fallback_emoji: Option<String>
}


impl Values {
    pub fn equals(&self, other: &Values) -> bool {
        self.match_text == other.match_text
    }
    pub fn from_hashmap(match_text: String, mut hashmap: HashMap<String, String>
    ) -> fancy_regex::Result<Values> {
        if let Some(text) = hashmap.remove("text") {
            return Ok(Values {
                match_regex: Regex::new(&match_text)?,
                match_text,
                text: Some(text),
                emoji: hashmap.remove("emoji"),
                regex_compiled: None,
                regex: None,
                format: None,
                fallback: None,
                fallback_emoji: None
            })
        }
        match (hashmap.remove("regex"), hashmap.remove("format")) {
            (Some(regex), Some(format)) => {
                Ok(Values {
                    match_regex: Regex::new(&match_text)?,
                    match_text,
                    text: None,
                    emoji: hashmap.remove("emoji"),
                    regex_compiled: Some(Regex::new(&regex)?),
                    regex: Some(regex),
                    format: Some(format),
                    fallback: hashmap.remove("fallback"),
                    fallback_emoji: hashmap.remove("fallback_emoji")
                })
            }
            _ => Err(fancy_regex::Error::ParseError)
        }
    }

    pub fn new(match_text: String,
               text: Option<String>,
               emoji: Option<String>,
               regex: Option<String>,
               format: Option<String>,
               fallback: Option<String>,
               fallback_emoji: Option<String>) -> Result<Values, fancy_regex::Error> {
        if text.is_some() {
            Ok(Values {
                match_regex: Regex::new(&match_text).unwrap(),
                match_text,
                text,
                emoji,
                regex: None,
                regex_compiled: None,
                format: None,
                fallback: None,
                fallback_emoji: None,
            })
        } else if regex.is_some() && format.is_some() {
            Ok(Values {
                match_regex: Regex::new(&match_text)?,
                match_text,
                text: None,
                emoji,
                regex_compiled: Some(Regex::new(&regex.as_ref().unwrap())?),
                regex,
                format,
                fallback,
                fallback_emoji,
            })
        } else {
            Err(fancy_regex::Error::UnknownFlag("incorrect fields set".to_string()))
        }
    }
}



fn main() {
    let ini_contents= std::fs::read_to_string(
        "config\\config.ini").unwrap();
    let token = std::fs::read_to_string("config\\token.txt")
        .unwrap();
    let (dict_commands, order) = parser::main(&ini_contents);
    let values: HashMap<String, Values> = dict_commands
        .into_iter()
        .map(|(k, v)| {
            match Values::from_hashmap(k.clone(), v) {
                Ok(x) => (k, x),
                Err(x) => panic!("{:?}", x)
            }})
        .collect();
    let order = order.iter().filter_map(|string| {
        // regex that doesn't compile is ignored
        Regex::new(string).ok()
    }).collect();
    let mut client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("Authorization", HeaderValue::from_str(&token).unwrap());
    let mut previous_running_app: Option<String> = None;
    let mut running_app: Option<(String, String)> = None;
    loop {
        // updates every 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));
        // get_apps::main has tons of unsafe calls
        if let Some(new_apps) = get_apps::main(&order) {
            match (&previous_running_app, &running_app) {
                (Some(x), _) if x == &new_apps.1 => {
                    continue;
                },
                (_, Some(_)) => {
                    previous_running_app = Some(running_app.take().unwrap().1);
                    running_app = Some(new_apps);
                }
                _ => {
                    running_app = Some(new_apps);
                },
            }
        }
        if let Some(x) = &running_app {
            println!("this app detected -> '{}'\nthis title detected -> '{}'",
                     x.1, x.0);
        } else {
            println!("no app detected");
        }
        client::main(&mut client, &values,
                     &running_app,
                     &headers);
    }

}
