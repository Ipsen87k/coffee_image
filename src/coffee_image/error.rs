use std::{io, num::ParseIntError};


#[derive(Debug,Clone)]
pub enum Error{
    DialogClosed,
    IOFailed(io::ErrorKind),
    ParseError(ParseIntError),
}