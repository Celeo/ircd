//! IRC protocol message parsing and serialization.

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(missing_docs)]

use std::collections::HashMap;

/// IRC message.
pub struct Message {
    /// Message metadata.
    pub tags: HashMap<String, Option<String>>,
    /// Message origin.
    pub source: Option<String>,
    /// IRC command or numeric.
    pub command: String,
    /// Command arguments.
    pub parameters: Vec<String>,
}

/// Protocol errors.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProtoError {
    //
}

///
pub fn parse_message(message: &str) -> Result<Message, ProtoError> {
    todo!()
}
