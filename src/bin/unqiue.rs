use flyio_dist::{Message, Node, Body, Payload, waitloop};
use serde::{Deserialize, Serialize };
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum PayloadGen {
    Generate {},
    GenerateOk { id: String },
}

struct IdNode { id: usize }

impl Node<PayloadGen> for IdNode {
    fn id(&self) -> usize {
        self.id
    }

    fn inc_id(&mut self) {
        self.id += 1        
    }

    fn send(&mut self, input: Message<PayloadGen>, out: &mut std::io::StdoutLock) {
        if let Some(e) = input.body.payload.extra() {
            match e {
                PayloadGen::Generate { .. } => {
                    let unique = format!("hi-{}-{}", input.dest, self.id);
                    let reply = Message {
                        src: input.dest,
                        dest: input.src,
                        body: Body { id: Some(self.id), in_reply_to: input.body.id, payload: Payload::Extra(PayloadGen::GenerateOk { id: unique  }) }
                    };
                    serde_json::to_writer(&mut *out, &reply).expect("could not serialize!");
                    out.write_all(b"\n").expect("could not write new line!");
                    self.id += 1; 
                },
                PayloadGen::GenerateOk { .. } => {},
            }
        }
    }
}

fn main() {
    let node = IdNode { id: 0 };
    waitloop(node);
}