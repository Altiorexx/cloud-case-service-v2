use std::sync::Arc;
use rocket::{futures::stream::SplitSink, tokio::sync::RwLock};
use rocket_ws::{stream::DuplexStream, Message};
use std::collections::HashMap;


pub struct SocketService {
    cases: Arc<RwLock<HashMap<String, Vec<String>>>>, // users for an individual case
    clients: Arc<RwLock<HashMap<String, SplitSink<DuplexStream, Message>>>> // each client's connection
}

pub async fn new_socket_service() -> SocketService {
    SocketService {
        cases: Arc::new(RwLock::new(HashMap::new())),
        clients: Arc::new(RwLock::new(HashMap::new()))
    }
}

// connect
// check if list already exists for case
// add user to case list
// store user client info (id, socket)
// receive messages and update case
// *broadcast to all on case list, except producer


impl SocketService {

    pub async fn active_client(&self, case_id: &String) -> bool {
        let cases = self.cases.read().await;
        cases.contains_key(case_id)
    }

    // stores a client's connection info (user_id, socket).
    pub async fn add_client(&self, user_id: &String, sender: SplitSink<DuplexStream, Message>) {
        let mut clients = self.clients.write().await;
        clients.insert(user_id.to_string(), sender);
    }

    // checks if a case list already exists, otherwise creates one.
    pub async fn add_client_to_case_list(&self, case_id: &String, user_id: &String) {
        let mut cases = self.cases.write().await;
        if let Some(case_list) = cases.get_mut(case_id) {
            case_list.push(user_id.to_string())
        } else {
            cases.insert(case_id.to_string(), vec![user_id.to_string()]);
        }
    }

    // cleans up after a client. this includes removing their connection details and from their associated case.
    pub async fn remove_client(&self, user_id: String, case_id: String) {

        // remove client from clients (curly brackets is some kind of scoping, minimizing the lock on the hashmap?)
        {
            let mut clients = self.clients.write().await;
            if clients.remove(&user_id).is_some() {
                println!("removed client {} from clients", user_id);
            } else {
                println!("client {} was not found", user_id);
            }
        }

        // remove user_id from cases
        let mut cases = self.cases.write().await;
        if let Some(case) = cases.get_mut(&case_id) {
            case.retain(|id| id != &user_id);
            println!("removed client {} from case {}", user_id, case_id);
            // if case list is empty afterwards, delete case
            if case.is_empty() {
                cases.remove(&case_id);
                println!("case list is empty and has been removed");
            }
        } else {
            println!("case {} was not found", case_id);
        }
    }

    pub async fn print_clients(&self) {
        let clients = self.clients.read().await;
        println!("connected clients: ");
        for user_id in clients.keys() {
            println!("user_id: {}", user_id);
        }
    }

    pub async fn print_case_lists(&self) {
        let cases = self.cases.read().await;
        for (case_id, user_id_list) in cases.iter() {
            println!("case_id: {}", case_id);
            for user_id in user_id_list {
                println!("user_id: {}", user_id);
            }
        }
    }

}