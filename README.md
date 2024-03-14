## Simple distributed system in Rust
### Implementation of first 3 challanges provided by Fly.io (https://fly.io/dist-sys/)
#### To make it work you need to have maelstrom installed

* ### Echo - "ping-ponging" all echo messages between nodes
    1. `cargo build --release`
    2. `mv target/release/echo .`
    3. `maelstrom test -w echo --bin ./echo --node-count 1 --time-limit 10`
* ### Unique ID - generating all node unique IDs for messages
    1. `cargo build --release`
    2. `mv target/release/unique .`
    3. `maelstrom test -w unique-ids --bin unique --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition`
* ### Broadcast - broadcasting recieved messages to all nodes in same cluster
    1. `cargo build --release`
    2. `mv target/release/broadcast .`
    3. `maelstrom test -w broadcast --bin broadcast --node-count 5 --time-limit 20 --rate 10`