mod server;
mod client;

fn main() {
    let line = String::new();
    println!("Act as a client? [Y]/n: ");
    match std::io::stdin().read_line(&mut line).unwrap() {
        "Y" => client::client(),
        "y" => client::client(),
        _ => server::server()
    }
}
