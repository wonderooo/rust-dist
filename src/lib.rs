use std::io::{StdoutLock, Write};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<P> {
    pub src: String,
    pub dest: String,
    pub body: Body<P>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<P> {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload<P>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Payload<P> {
    Echo { echo: String },
    EchoOk { echo: String },
    Init { node_id: String, node_ids: Vec<String> },
    InitOk {},
    #[serde(untagged)]
    Extra(P),
}

impl<P> Payload<P> {
    pub fn extra(self) -> Option<P> {
        if let Self::Extra(inner) = self {
            return Some(inner)
        }
        None
    }
}

pub trait Node<P> where P: Serialize {
    fn id(&self) -> usize;
    fn inc_id(&mut self);
    fn send(&mut self, input: Message<P>, out: &mut StdoutLock);
    fn send_base(&mut self, input: Message<P>, out: &mut StdoutLock) {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body { id: Some(self.id()), in_reply_to: input.body.id, payload: Payload::<P>::EchoOk { echo } }
                };
                serde_json::to_writer(&mut *out, &reply).expect("could not serialize!");
                out.write_all(b"\n").expect("could not write new line!");
                self.inc_id();
            },
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body { id: Some(self.id()), in_reply_to: input.body.id, payload: Payload::<P>::InitOk {  } }
                };
                serde_json::to_writer(&mut *out, &reply).expect("could not serialize!");
                out.write_all(b"\n").expect("could not write new line!");
                self.inc_id();
            },
            Payload::EchoOk { .. } | Payload::InitOk { } => {},
            Payload::Extra( .. ) => self.send(input, out),
        }
    }
}


pub fn waitloop<'a, P>(mut node: impl Node<P>) where P: Deserialize<'a> + Serialize {
    let stdin = std::io::stdin().lock();
    let ind = serde_json::Deserializer::from_reader(stdin).into_iter::<Message::<P>>();

    let mut stdout = std::io::stdout().lock();

    for i in ind {
        let i = i.expect("Could not serialize from stdin!");
        node.send_base(i, &mut stdout);
    }
}
