use rocket_ws as ws;
use rocket::State;
use rocket::futures::StreamExt;
use std::sync::Arc;

use crate::api::middleware_handler::AuthorizeClientGuard;
use crate::database::case::CaseDatabase;
use crate::service::socket::SocketService;
use crate::service::user::UserService;
use crate::types::collaboration_handler::Message;



#[get("/api/collaboration/case/<case_id>/connect/<user_id>?<token>")]
pub async fn connect<'a>(
    _guard: AuthorizeClientGuard,
    case_database: &'a State<CaseDatabase>,
    user_service: &'a State<Arc<UserService>>,
    socket_service: &'a State<SocketService>,
    ws: ws::WebSocket,
    case_id: String,
    user_id: String,
    token: String
) -> ws::Channel<'a> {
   
    println!("client connected to case {}", case_id);

    match user_service.check(token).await {
        Ok(_) => println!("token verified for collaboration access"),
        Err(e) => {
            eprintln!("error checking client token: {}", e);
            return ws.channel(move |_stream| Box::pin(async { Ok(()) }));
        }
    };

    // check if there is already a user connected to the case
    if socket_service.active_client(&case_id).await {
        return ws.channel(move |_stream| Box::pin(async { Ok(()) }));
    }

    // find out what type of case it is, so it is possible to decide how to handle received changes
    let framework = match case_database.read_case_framework(&case_id).await {
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

        socket_service.add_client(&user_id, sender).await;
        println!("added client connection to registry");

        socket_service.add_client_to_case_list(&case_id, &user_id).await;
        println!("added client to case list");

        while let Some(message) = receiver.next().await {
            match message {
                Ok(msg) => {
                    if let rocket_ws::Message::Text(v) = msg {
                        match serde_json::from_str::<Message>(&v) {
                            Ok(parsed) => {
                                if framework == "cis18" {
                                    match case_database.update_cis18_content(&case_id, &parsed).await {
                                        Ok(_) => {
                                            println!("updated cis18 content!");
                                            ()
                                        },
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
        socket_service.remove_client(user_id, case_id).await;
        Ok(())
    }))
}




