pub mod parse;
pub mod raw;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate derivative;
extern crate serde_json;

use serde_json::from_reader;
use std::fs::File;
use std::io::Read;
