use std::fmt;

#[derive(Clone)]
pub struct Request {
    pub location: String,
    pub target: String,
    pub path: String,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Request {{ location: {}, target: {}, path, {} }}",
            self.location, self.target, self.path
        )
    }
}
