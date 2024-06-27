use rocket_ws as ws;
use rocket::State;
use rocket::futures::StreamExt;

use crate::database::case::{new_case_database, CaseDatabase};
use crate::service::socket::{SocketService, new_socket_service};
use crate::types::collaboration_handler::Message;



pub struct CollaborationHandler {
    socket: SocketService,
    case: CaseDatabase
}

pub async fn new_collaboration_handler() -> CollaborationHandler {
    CollaborationHandler {
        socket: new_socket_service().await,
        case: new_case_database().await
    }
}

#[get("/api/collaboration/<case_id>/connect/<user_id>")]
pub async fn connect(handler: &State<CollaborationHandler>, ws: ws::WebSocket, case_id: String, user_id: String) -> ws::Channel<'_> {
    println!("client connected to case {}", case_id);
    
    // check if there is already a user connected to the case
    if handler.socket.active_client(&case_id).await {
        return ws.channel(move |_stream| Box::pin(async { Ok(()) }));
    }

    // find out what type of case it is, so it is possible to decide how to handle received changes
    let framework = match handler.case.read_case_framework(&case_id).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error reading case framework: {}", e);
            return ws.channel(move |_stream| Box::pin(async { Ok(()) }));
        }
    };
    
    let ws = ws.config(ws::Config {
        ..Default::default()
    });

    ws.channel(move |stream| Box::pin(async move {
        
        let (sender, mut receiver) = stream.split();

        handler.socket.add_client(&user_id, sender).await;
        println!("added client connection to registry");

        handler.socket.add_client_to_case_list(&case_id, &user_id).await;
        println!("added client to case list");

        while let Some(message) = receiver.next().await {
            match message {
                Ok(msg) => {
                    if let rocket_ws::Message::Text(v) = msg {
                        match serde_json::from_str::<Message>(&v) {
                            Ok(parsed) => {
                                if framework == "cis18" {
                                    match handler.case.update_cis18_content(&case_id, &parsed).await {
                                        Ok(_) => (),
                                        Err(e) => {
                                            eprintln!("error updating cis18 case: {}", e);
                                            break;
                                        }
                                    }
                                }
                                // then the message should be broadcasted here
                            },
                            Err(e) => eprintln!("error parsing received message: {}", e)
                        }
                    } else if let rocket_ws::Message::Close(_) = msg {
                        println!("client disconnected")
                    }
                },
                Err(e) => {
                    println!("error receiving messages: {}", e);
                    break;
                }
            }
        }
        handler.socket.remove_client(user_id, case_id).await;
        Ok(())
    }))
}




