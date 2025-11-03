use std::fmt;

#[allow(dead_code)]
pub enum Protocol {
    Http,
    Https,
}

impl fmt::Display for Protocol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Protocol::Http => write!(formatter, "Http"),
            Protocol::Https => write!(formatter, "Https"),
        }
    }
}

pub fn format_hostname(protocol: Protocol, raw: String) -> String {
    format!("{}://{}", protocol.to_string().to_lowercase(), raw)
}

pub fn pad_string(string: &str, padding: i32) -> String {
    let mut new_string = String::new();

    new_string.push_str(string);

    for _ in 0..padding {
        new_string.push_str("   ");
    }

    new_string
}