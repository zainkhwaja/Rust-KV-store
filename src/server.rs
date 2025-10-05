use crate::storage::Storage;
use crate::commands::{Command, create_response};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use anyhow::Result;
use log::{info, error};

pub struct Server {
    storage: Storage,
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server {
            storage: Storage::new(),
            address,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        info!("Server listening on {}", self.address);

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    info!("New connection from {}", addr);
                    let storage = self.storage.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(socket, storage).await {
                            error!("Error handling connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

async fn handle_connection(mut socket: TcpStream, storage: Storage) -> Result<()> {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).await? == 0 {
            break;
        }

        let response = match Command::parse(line.as_bytes()) {
            Ok(Command::Get { key }) => {
                create_response(storage.get(&key))
            }
            Ok(Command::Set { key, value }) => {
                storage.set(key, value)?;
                b"+OK\r\n".to_vec()
            }
            Ok(Command::Delete { key }) => {
                if storage.delete(&key) {
                    b":1\r\n".to_vec()
                } else {
                    b":0\r\n".to_vec()
                }
            }
            Err(e) => {
                format!("-ERR {}\r\n", e).into_bytes()
            }
        };

        writer.write_all(&response).await?;
        writer.flush().await?;
    }

    Ok(())
}