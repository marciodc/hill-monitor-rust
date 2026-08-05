use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use hill_common::event::{get_event_bus, TipoEvento};
use tracing::{error, info};

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    info!("Nova conexão WebSocket estabelecida.");

    let mut rx = get_event_bus().subscribe();

    loop {
        tokio::select! {
            event_result = rx.recv() => {
                match event_result {
                    Ok(app_msg) => {
                        match app_msg.tipo {
                            TipoEvento::EvtStatusAbastecimento => {
                                if let Err(e) = socket.send(Message::Text(app_msg.mensagem.into())).await {
                                    error!("Erro ao enviar mensagem no WebSocket: {:?}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                        continue;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            client_msg = socket.recv() => {
                match client_msg {
                    Some(Ok(msg)) => {
                        match msg {
                            Message::Close(_) => break,
                            Message::Ping(p) => {
                                if let Err(e) = socket.send(Message::Pong(p)).await {
                                    error!("Erro ao responder Pong: {:?}", e);
                                    break;
                                }
                            }
                            Message::Text(t) => {
                                if let Err(e) = socket.send(Message::Text(t)).await {
                                    error!("Erro ao ecoar mensagem: {:?}", e);
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
    }

    info!("Conexão WebSocket encerrada.");
}
