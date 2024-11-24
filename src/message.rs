use std::fmt;
use std::path::PathBuf;

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    AnyNA {
        filename: PathBuf,
        location: Location,
    },
}

impl Message {
    /// Short ID for the message.
    pub fn code(&self) -> &'static str {
        match self {
            Message::AnyNA { filename: _, location: _ } => "any-na",
        }
    }

    /// The body text for the message.
    pub fn body(&self) -> &'static str {
        match self {
            Message::AnyNA { filename: _, location: _ } => "Inefficient: use anyNA() instead.",
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::AnyNA { filename, location } => write!(
                f,
                "{}{}{}{}{}  {}\t{}",
                filename.to_string_lossy().white().bold(),
                ":  ".cyan(),
                location.column,
                ":".cyan(),
                location.row,
                self.code().red().bold(),
                self.body()
            ),
        }
    }
}
