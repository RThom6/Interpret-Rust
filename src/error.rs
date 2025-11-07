pub struct Error {
    message: String,
    trace: String,
    line: u64,
}

impl Error {
    pub fn new(message: String, line: u64) -> Self {
        Error {
            message,
            trace: String::new(),
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}\n\t{}", self.message, self.trace)
    }
}

pub fn error(message: String, line: u64) -> Error {
    Error::new(message, line)
}

pub fn error_with_trace(message: String, line: u64, mut error: Error) -> Error {
    error.trace = format!("{}\n{}: at line {}", error.trace, message, line);
    error
}

fn report(error: Error) {
    eprintln!()
}
