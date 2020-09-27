# Just another Rust-WebSocket Application

This project aims to create a rust-lang chat application to communicate between two nodes using the websocket standard.

## Why Rust?
The use of a memory safe programming language has not been well researched in the domain of websockets and I believe there is a vast array of capabilities and performance upgrades that can be made available to a chat program by using rust. The learnings from this project is to be utilised in future application projects that will handle data of varying nature, starting most probably with a **Voice over WebSocket** calling application which is also being planned. Also, as a beginner in Rust, I am extremely curious about how complex such a project can get.

## Installation and Working
The program makes use of tungstenite to manage and make use of the websocket communication protocol. To download the repo and demo the code, please follow the steps:
1. Download the repo from GitHub:
```bash
git clone https://github.com/de-sh/wschatrs && cd wschatrs
```
2. Next, setup a server instance. We are currently one directional and need to build on this to further our understanding of the subject:
```bash
cargo run --bin server
```
Now the terminal would have shown compilation info, but nothing more, A message would also be printed to show that the server is up and running.
3. Finally, in order to setup a client for communicating with the server, use the following command:
```bash
cargo run --bin client
```
The following information will be printed:
- Server:
```
Received a new ws handshake
The request's path is: /socket
The request's headers are:
* host
* connection
* upgrade
* sec-websocket-version
* sec-websocket-key
```
- Client:
```
Connected to the server
Response HTTP code: 101 Switching Protocols
Response contains the following headers:
* connection
* upgrade
* sec-websocket-accept
* mycustomheader
* some_tungstenite_header
Received: Hello WebSocket
```


## Contribution
All contributions in any form are welcome. Especially in the form of guidance, please open an issue if you find something wrong with the code and can help direct the project, but otherwise this is a learning journey I wish to make on my own.
