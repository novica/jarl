use std::rc::Rc;

use crate::parser::Localization;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Session {
    pub locale: Localization,
    pub warranty: bool,
    pub history: Option<String>,
    pub output: SessionOutput,
}

pub enum SessionOutput {
    Stdout(std::io::Stdout),
    Callback(Rc<dyn Fn(String)>),
}

// A subset of the Session info that is thread-safe for passing to reedline::{Validator, Highlighter}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SessionParserConfig {
    pub locale: Localization,
}

impl From<Session> for SessionParserConfig {
    fn from(val: Session) -> Self {
        SessionParserConfig { locale: val.locale }
    }
}

impl std::fmt::Debug for SessionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionOutput::Stdout(s) => s.fmt(f),
            SessionOutput::Callback(_) => write!(f, "SessionOutput::Callback"),
        }
    }
}

impl Default for SessionOutput {
    fn default() -> Self {
        SessionOutput::Stdout(std::io::stdout())
    }
}

impl Clone for SessionOutput {
    fn clone(&self) -> Self {
        match self {
            Self::Stdout(_) => Self::Stdout(std::io::stdout()),
            Self::Callback(f) => Self::Callback(f.clone()),
        }
    }
}

impl PartialEq for SessionOutput {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (Self::Stdout(_), Self::Stdout(_)))
    }
}

impl std::io::Write for SessionOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Stdout(s) => s.write(buf),
            Self::Callback(f) => {
                let len = buf.len();
                f(std::str::from_utf8(buf).unwrap_or_default().to_string());
                Ok(len)
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Stdout(s) => s.flush(),
            Self::Callback(_) => Ok(()),
        }
    }
}

impl Session {
    pub fn with_history_file(mut self, file: String) -> Session {
        self.history = Some(file);
        self
    }

    pub fn with_output(mut self, output: SessionOutput) -> Session {
        self.output = output;
        self
    }
}
