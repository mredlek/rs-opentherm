#![deny(missing_docs)]

//! This crates deals with the content of the OpenTherm protocol

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate error_chain;

mod application;
mod conversation;
mod error;
mod message;

pub use error::{Error, ErrorKind};
pub use message::*;
pub use conversation::{Conversation, NullableComplexType};
pub use application::*;
