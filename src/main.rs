use std::io::{self, BufRead};

use packet::write_to_stdout;
use packet::Body;
use packet::Message;
use uuid::Uuid;
use tokio;

mod packet;

#[tokio::main]
async fn main() {
    let mut node_id = String::new();

    for line in io::stdin().lock().lines() {
        let input: Message = serde_json::from_str(&line.unwrap()).unwrap();
        match input.body {
            Body::Init {
                msg_id,
                node_id: id,
                ..
            } => {
                // Setting this node id
                node_id = id;

                let output = Message {
                    src: node_id.clone(),
                    dest: input.src,
                    body: Body::InitOk {
                        in_reply_to: msg_id,
                    },
                };

                tokio::spawn(async { write_to_stdout(output) });
            }
            Body::Generate { msg_id} => {
                let output = Message {
                    src: node_id.clone(),
                    dest: input.src,
                    body: Body::GenerateOk {
                        msg_id: msg_id,
                        in_reply_to: msg_id,
                        id: Uuid::new_v4().to_string()
                    },
                };

                tokio::spawn(async { write_to_stdout(output) });
            }
            Body::Error {
                in_reply_to,
                code,
                text,
            } => {
                eprintln!(
                    "Error received (in_reply_to: {}, code: {}, text: {})",
                    in_reply_to, code, text
                );
            }
            _ => (),
        }
    }
}
