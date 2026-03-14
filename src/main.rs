use my_protected_crate::ProtectedClient;

fn main() {
    let client = match ProtectedClient::new() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("License verification failed: {e}");
            std::process::exit(1);
        }
    };

    println!("Licensed to: {}", client.customer_id());
    println!("{}", client.do_something());
}
