use std::fmt;

pub const LICENSE_MESSAGE: &str = r#"LoadThing Copyright (C) 2025 Kinetic Labs <https://github.com/Kinetic-Labs>
This program comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it under certain conditions.

Please see GNU GPL-3 <https://www.gnu.org/licenses/gpl-3.0.en.html> for more details"#;

pub struct Request {
    pub location: String,
    pub target: String,
    pub path: String,
    pub ip: String,
    pub time: u128,
}

impl fmt::Display for Request {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Request {{ location: {}, target: {}, path: {}, ip: {} time: {}ms }}",
            self.location, self.target, self.path, self.ip, self.time
        )
    }
}
