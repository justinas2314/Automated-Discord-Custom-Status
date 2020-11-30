use std::collections::HashMap;

pub fn main(text: &str) -> (HashMap<String, HashMap<String, String>>, Vec<&str>) {
    let mut buffer = Vec::new();
    let mut commands = HashMap::new();
    let mut order = Vec::new();
    for line in text.lines() {
        if line.trim().len() == 0 {
            continue;
        }
        match &line[0..1] {
            ";" => continue,
            "\"" => {
                order.push(line.trim().trim_matches('"'));
                if buffer.len() != 0 {
                    let (name, dict) = parse(&buffer);
                    commands.insert(name, dict);
                }
                buffer.clear();
                buffer.push(line);
            },
            _ => {buffer.push(line)}
        }
    }
    if buffer.len() != 0 {
        let (name, dict) = parse(&buffer);
        commands.insert(name, dict);
    }
    (commands, order)
}


fn parse(lines: &Vec<&str>) -> (String, HashMap<String, String>) {
    // this is a mess
    // but if it works it works
    let mut dict = HashMap::new();
    let mut name = String::new();
    for line in lines.iter() {
        if line.trim().len() == 0 {
            continue;
        }
        if "\"" == &line[0..1] {
            let mut last_index = 0;
            for (i, j) in line.chars().enumerate() {
                match j {
                    '"' => { last_index = i},
                    _ => (),
                }
            }
            name.push_str(&line[1..last_index]);
        } else {
            let (key, value) = get_kv(line);
            dict.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    (name, dict)
}


fn get_kv(line: &str) -> (String, String) {
    let mut key = String::new();
    let mut value = String::new();
    let mut switched = false;
    let mut escape = false;
    for i in line.chars() {
        match i {
            '\\' if !escape => {escape = true},
            '=' if !escape => {switched = true},
            x if !switched => {key.push(x); escape = false},
            x if switched => {value.push(x); escape = false},
            _ => ()
        }
    }
    (key, value)
}
