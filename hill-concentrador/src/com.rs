use std::io::{Read, Write};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info, warn};

pub struct CommandRequest {
    pub command: String,
    pub expect_response: bool,
    pub response_tx: Option<oneshot::Sender<String>>,
}

#[derive(Clone)]
pub struct ConcentradorCom {
    tx: mpsc::Sender<CommandRequest>,
}

impl ConcentradorCom {
    pub fn new(port_name: &str) -> Self {
        let (tx, mut rx) = mpsc::channel::<CommandRequest>(32);
        let port_name_str = port_name.to_string();

        tokio::spawn(async move {
            let mut port = match serialport::new(&port_name_str, 115_200)
                .timeout(Duration::from_millis(100))
                .open()
            {
                Ok(p) => p,
                Err(e) => {
                    error!("Erro ao abrir a porta serial {}: {:?}", port_name_str, e);
                    return;
                }
            };

            info!("Porta serial {} aberta com sucesso.", port_name_str);

            let mut response_buffer = String::new();

            while let Some(req) = rx.recv().await {
                if let Err(e) = port.write_all(req.command.as_bytes()) {
                    error!("Erro ao enviar comando serial: {:?}", e);
                    continue;
                }

                if req.expect_response {
                    let mut temp_buf = [0; 256];
                    response_buffer.clear();
                    let start = std::time::Instant::now();
                    let timeout = Duration::from_secs(5);

                    loop {
                        if start.elapsed() > timeout {
                            warn!("Timeout aguardando resposta para o comando: {}", req.command);
                            break;
                        }

                        match port.read(&mut temp_buf) {
                            Ok(n) if n > 0 => {
                                if let Ok(s) = std::str::from_utf8(&temp_buf[..n]) {
                                    response_buffer.push_str(s);
                                    if response_buffer.ends_with(')') {
                                        break;
                                    }
                                }
                            }
                            Ok(_) => {}
                            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                                // Ignore timeout and loop again
                            }
                            Err(e) => {
                                error!("Erro lendo da porta serial: {:?}", e);
                                break;
                            }
                        }
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }

                    if let Some(tx) = req.response_tx {
                        let _ = tx.send(response_buffer.clone());
                    }
                }
            }
        });

        Self { tx }
    }

    pub async fn send_command(&self, command: &str, expect_response: bool) -> String {
        let (response_tx, response_rx) = oneshot::channel();
        let req = CommandRequest {
            command: command.to_string(),
            expect_response,
            response_tx: if expect_response { Some(response_tx) } else { None },
        };

        if let Err(e) = self.tx.send(req).await {
            error!("Erro ao enfileirar comando serial: {:?}", e);
            return String::new();
        }

        if expect_response {
            response_rx.await.unwrap_or_default()
        } else {
            String::new()
        }
    }
}
