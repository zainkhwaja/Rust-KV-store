use anyhow::{anyhow, Result};
use bytes::Bytes;

#[derive(Debug)]
pub enum Command {
    Set { key: String, value: Vec<u8> },
    Get { key: String },
    Delete { key: String },
}

impl Command {
    pub fn parse(input: &[u8]) -> Result<Command> {
        let input = String::from_utf8_lossy(input);
        let mut parts = input.trim().split_whitespace();
        
        match parts.next().map(|s| s.to_uppercase()) {
            Some(cmd) => match cmd.as_str() {
                "SET" => {
                    let key = parts.next().ok_or_else(|| anyhow!("Missing key"))?.to_string();
                    let value = parts.next().ok_or_else(|| anyhow!("Missing value"))?.as_bytes().to_vec();
                    Ok(Command::Set { key, value })
                }
                "GET" => {
                    let key = parts.next().ok_or_else(|| anyhow!("Missing key"))?.to_string();
                    Ok(Command::Get { key })
                }
                "DEL" => {
                    let key = parts.next().ok_or_else(|| anyhow!("Missing key"))?.to_string();
                    Ok(Command::Delete { key })
                }
                _ => Err(anyhow!("Unknown command")),
            },
            None => Err(anyhow!("Empty command")),
        }
    }
}

pub fn create_response(content: Option<Vec<u8>>) -> Vec<u8> {
    match content {
        Some(value) => {
            let mut response = b"$".to_vec();
            response.extend_from_slice(value.len().to_string().as_bytes());
            response.extend_from_slice(b"\r\n");
            response.extend_from_slice(&value);
            response.extend_from_slice(b"\r\n");
            response
        }
        None => b"$-1\r\n".to_vec(),
    }
}