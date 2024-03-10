use flyio_dist::{Message, Node, waitloop};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum PayloadEcho{}

struct EchoNode {
    id: usize,
}

impl Node<PayloadEcho> for EchoNode {
    fn id(&self) -> usize {
        self.id
    }

    fn inc_id(&mut self) {
        self.id += 1
    }

    fn send(&mut self, _: Message<PayloadEcho>, _: &mut std::io::StdoutLock) {
        
    }
}

fn main() {
    let node = EchoNode { id: 0 };
    waitloop(node);
}