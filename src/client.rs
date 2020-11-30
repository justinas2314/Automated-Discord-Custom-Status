use std::collections::HashMap;
use fancy_regex::Regex;
use serde_json::Value;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::Values;


struct StatusWrapper {
    custom_status: StatusJson
}




struct StatusJson {
    emoji_name: Option<String>,
    text: Option<String>
}


impl Serialize for StatusJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let length = match (&self.text, &self.emoji_name) {
            (Some(_), Some(_)) => 2,
            (None, None) => 0,
            _ => 1
        };
        let mut state = serializer.serialize_struct("custom_status", length)?;
        if let Some(x) = &self.text {
            state.serialize_field("text", x)?;
        }
        if let Some(x) = &self.emoji_name {
            state.serialize_field("emoji_name", x)?;
        }
        state.end()
    }
}


impl Serialize for StatusWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("custom_status", 1)?;
        state.serialize_field("custom_status", &self.custom_status)?;
        state.end()
    }
}


fn format(expression: &Regex, text: &str, format_str: &str) -> String {
    let mut format_string = format_str.to_string();
    if let Ok(Some(capture)) = expression.captures(text) {
        for i in 0..capture.len() {
            format_string = format_string.replace(&(
                "{".to_string() + &i.to_string() + "}"), &capture.get(i).unwrap().as_str());
        }
    }
    format_string
}


fn function(client: &mut Client, vector: (&str, &str),
            values: &Values,
            headers: &HeaderMap) {
    // no i'm not yandere dev
    let mut text = match &values.text {
        Some(x) => Some(x.to_string()),
        None => None
    };
    let mut emoji = match &values.emoji {
        Some(x) => Some(x.to_string()),
        None => None
    };
    match (&values.regex_compiled,
           &values.format) {
        (Some(a), Some(b)) => {
            let s = format(a, &vector.1, b);
            if b == &s {
                if let Some(x) = &values.fallback {
                    text = Some(x.to_string());
                }
                if let Some(x) = &values.fallback_emoji {
                    emoji = Some(x.to_string());
                }
            } else {
                text = Some(s);
            }
        },
        _ => ()
    };
    if let Some(s) = &text {
        if s.len() > 128 {
            if let Some(x) = &values.fallback {
                text = Some(x.to_string());
            } else {
                text = Some(too_long(s).to_string());
            }
            if let Some(x) = &values.fallback_emoji {
                emoji = Some(x.to_string());
            }
        }
    }

    let json = StatusWrapper {
        custom_status: StatusJson {
            emoji_name: emoji,
            text
        }
    };
    let status_code = client
        .patch("https://discord.com/api/v6/users/@me/settings")
        .headers(headers.clone())
        .json(&json)
        .send()
        .unwrap()
        .status();
    println!("status code {}", status_code);
}


fn clear(client: &mut Client, headers: &HeaderMap) {
    let status_code = client
        .patch("https://discord.com/api/v6/users/@me/settings")
        .headers(headers.clone())
        .json(&serde_json::from_str::<Value>("{\"custom_status\": null}").unwrap())
        .send()
        .unwrap()
        .status();
    println!("status code {}", status_code);
}


fn too_long(text: &str) -> &str {
    let mut output_length = 0;
    for i in text.split(" ") {
        if output_length + i.len() > 128 {
            break;
        } else {
            output_length += i.len() + 1; // to account for the split spaces
        }
    }
    &text[0..output_length]
}


pub fn main(client: &mut Client, commands: &HashMap<String, Values>,
            parsed_input: &Option<(String, String)>,
            headers: &HeaderMap) {
    // the details and state values cannot be less than 4 or more than 29
    // the image texts cannot be longer than 128
    // all of these will change to "    " or "" to prevent crashes
    match parsed_input {
        Some((x, y)) => {
            match commands.get(x) {
                Some(comms) => function(client, (x, y), comms, headers),
                _ => clear(client, headers)
            }
        }
        _ => clear(client, headers)
    }
}
