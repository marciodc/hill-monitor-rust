use axum::{
    Json,
    extract::Request,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use hill_common::event::{TipoEvento, get_event_bus};
use tracing::{error, info};

use crate::web::auth;
use crate::web::service::response::ApiResponse;

pub async fn ws_handler(ws: WebSocketUpgrade, request: Request) -> Response {
    if !auth::is_valid_authorization(request.headers().get(header::AUTHORIZATION)) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<()>::err("Token inválido")),
        )
            .into_response();
    }

    ws.on_upgrade(|socket| handle_socket(socket))
        .into_response()
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
