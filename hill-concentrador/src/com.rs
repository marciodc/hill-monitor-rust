use std::io::{Read, Write};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info, warn};

fn adiciona_check(st: &str) -> String {
    let mut acumulador: u8 = 0;
    for c in st.chars() {
        acumulador = acumulador.wrapping_add(c as u8);
    }
    format!("({}{:02X})", st, acumulador)
}

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
            let mut response_buffer = String::new();
            loop {
                let mut port = match serialport::new(&port_name_str, 115_200)
                    .timeout(Duration::from_millis(100))
                    .open()
                {
                    Ok(p) => {
                        info!("Porta serial {} aberta com sucesso.", port_name_str);
                        p
                    }
                    Err(e) => {
                        error!("Erro ao abrir a porta serial {}: {:?}", port_name_str, e);
                        tokio::time::sleep(Duration::from_secs(3)).await;
                        continue;
                    }
                };

                let mut reconnect_required = false;

                while let Some(req) = rx.recv().await {
                    if let Err(e) = port.write_all(req.command.as_bytes()) {
                        error!("Erro ao enviar comando serial: {:?}", e);
                        reconnect_required = true;
                    } else if req.expect_response {
                        let mut temp_buf = [0; 256];
                        response_buffer.clear();
                        let start = std::time::Instant::now();
                        let timeout = Duration::from_secs(5);

                        loop {
                            if start.elapsed() > timeout {
                                warn!(
                                    "Timeout aguardando resposta para o comando: {}",
                                    req.command
                                );
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
                                    reconnect_required = true;
                                    break;
                                }
                            }
                            tokio::time::sleep(Duration::from_millis(20)).await;
                        }

                        if let Some(tx) = req.response_tx {
                            let _ = tx.send(response_buffer.clone());
                        }
                    }

                    if reconnect_required {
                        warn!(
                            "Reconectando porta serial {} após falha de comunicação.",
                            port_name_str
                        );
                        break;
                    }
                }

                if rx.is_closed() {
                    break;
                }
            }
        });

        Self { tx }
    }

    async fn enqueue_command(&self, command: String, expect_response: bool) -> String {
        let (response_tx, response_rx) = oneshot::channel();
        let req = CommandRequest {
            command,
            expect_response,
            response_tx: if expect_response {
                Some(response_tx)
            } else {
                None
            },
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

    pub async fn send_command(&self, command: &str, expect_response: bool) -> String {
        self.enqueue_command(command.to_string(), expect_response)
            .await
    }

    pub async fn request_status_bicos(&self) -> String {
        self.enqueue_command("(&S)".to_string(), true).await
    }

    pub async fn request_encerrante(&self, bico: &str) -> String {
        let cmd_body = format!("&T{}L", bico);
        let command = adiciona_check(&cmd_body);
        self.enqueue_command(command, true).await
    }

    pub async fn request_atualiza_preco(&self, bico: &str, tipo: i32, valor: &str) -> String {
        let cod_bico = format!("U{}", bico);
        let cmd_body = format!("&{}{}{}0{}", cod_bico, tipo, 0, valor);
        let command = adiciona_check(&cmd_body);
        self.enqueue_command(command, true).await
    }
}
