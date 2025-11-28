pub struct Error {
    message: &'static str,
    trace: String,
    line: u64,
}

impl Error {
    // TODO: String manipulation for useful error messages that point out where in a line an error occurred
    // TODO: Full blown error reporter class for different reporting strategies
    pub fn new(message: &'static str, line: u64) -> Self {
        Self {
            message,
            trace: String::new(),
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}\n\t{}", self.message, self.trace)
    }
}

pub fn error(message: &'static str, line: u64) -> Error {
    Error::new(message, line)
}

pub fn error_with_trace(message: &'static str, line: u64, mut error: Error) -> Error {
    error.trace = format!("{}\n{}: at line {}", error.trace, message, line);
    error
}

pub fn report(error: Error) {
    eprintln!("{}", error.to_string());
}
