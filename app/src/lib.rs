extern crate serde_json;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct JsonRequest {
    pub req_type: String,
    pub host: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub activity_type: Option<String>,
    pub large_image: Option<String>,
    pub small_image: Option<String>,
    pub anilist: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Request {
    Valid(JsonRequest),
    Invalid(String),
}

pub fn read_input<R: Read>(mut input: R) -> io::Result<Request> {
    let length = input.read_u32::<NativeEndian>().unwrap();
    let mut buffer = vec![0; length as usize];
    input.read_exact(&mut buffer)?;

    match serde_json::from_slice::<JsonRequest>(&buffer) {
        Ok(json_request) => Ok(Request::Valid(json_request)),
        Err(_) => match String::from_utf8(buffer) {
            Ok(invalid_request) => Ok(Request::Invalid(invalid_request)),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        },
    }
}

pub fn write_output<W: Write>(mut output: W, value: &serde_json::Value) -> io::Result<()> {
    let msg = serde_json::to_string(value)?;
    let len = msg.len();
    // Chrome won't accept a message larger than 1MB
    if len > 1024 * 1024 {
        panic!("Message was too large, length: {}", len)
    }
    output.write_u32::<NativeEndian>(len as u32)?;
    output.write_all(msg.as_bytes())?;
    output.flush()?;
    Ok(())
}
