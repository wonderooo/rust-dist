use std::{collections::{HashMap, HashSet}, io::Write};
use flyio_dist::{waitloop, Body, Message, Node, Payload};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum PayloadBroadcast{
    Broadcast { message: usize },
    BroadcastOk { },
    Read { },
    ReadOk { messages: Vec<usize> },
    Topology {topology: HashMap<String, Vec<String>> },
    TopologyOk { },
}

struct BroadcastNode {
    id: usize,
    messages: Vec<usize>,
    known: HashSet<String>
}

impl Node<PayloadBroadcast> for BroadcastNode {
    fn id(&self) -> usize {
        self.id
    }

    fn inc_id(&mut self) {
        self.id += 1;
    }

    fn send(&mut self, input: Message<PayloadBroadcast>, out: &mut std::io::StdoutLock) {
        if let Some(e) = input.body.payload.extra() {
            match e {
                PayloadBroadcast::Broadcast { message } => {
                    self.messages.push(message);
                    for neighbour in &self.known {
                        let reply = Message {
                            src: input.dest.clone(),
                            dest: neighbour.clone(),
                            body: Body {
                                id: Some(self.id),
                                in_reply_to: input.body.id,
                                payload: Payload::Extra(PayloadBroadcast::Broadcast { message } )
                            }
                        };
                        serde_json::to_writer(&mut *out, &reply).expect("count not serialize!");
                        out.write_all(b"\n").expect("could not write new line!"); 
                        self.id += 1;
                    }
                    let reply = Message {
                        src: input.dest,
                        dest: input.src,
                        body: Body {
                            id: Some(self.id),
                            in_reply_to: input.body.id,
                            payload: Payload::Extra(PayloadBroadcast::BroadcastOk { } )
                        }
                    };
                    serde_json::to_writer(&mut *out, &reply).expect("count not serialize!");
                    out.write_all(b"\n").expect("could not write new line!"); 
                    self.id += 1;
                },
                PayloadBroadcast::Read { } => {
                    let reply = Message {
                        src: input.dest,
                        dest: input.src,
                        body: Body {
                            id: Some(self.id),
                            in_reply_to: input.body.id,
                            payload: Payload::Extra(PayloadBroadcast::ReadOk { messages: self.messages.clone() })
                        }
                    };
                    serde_json::to_writer(&mut *out, &reply).expect("count not serialize!");
                    out.write_all(b"\n").expect("could not write new line!");
                    self.id += 1;
                },
                PayloadBroadcast::Topology { topology } => {
                    for k in topology.keys() {
                        self.known.insert(k.clone());
                    }
                    let reply = Message {
                        src: input.dest,
                        dest: input.src,
                        body: Body {
                            id: Some(self.id),
                            in_reply_to: input.body.id,
                            payload: Payload::Extra(PayloadBroadcast::TopologyOk {  })
                        }
                    };
                    serde_json::to_writer(&mut *out, &reply).expect("count not serialize!");
                    out.write_all(b"\n").expect("could not write new line!");
                    self.id += 1;
                },
                PayloadBroadcast::BroadcastOk {  } | PayloadBroadcast::ReadOk { .. } | PayloadBroadcast::TopologyOk {  } => {}
            }
        }
    }
}

fn main() {
    let node = BroadcastNode {
        id: 0,
        messages: vec![],
        known: HashSet::new(),
    };
    waitloop(node);
}