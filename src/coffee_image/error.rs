use std::io;


#[derive(Debug,Clone)]
pub enum Error{
    DialogClosed,
    IOFailed(io::ErrorKind),
}