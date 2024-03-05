use imap::*;

fn main() {
    let client = ClientBuilder::new("imap.mail.me.com",993)
        .mode(ConnectionMode::Tls)
        .connect()
        .unwrap();

    let username = match std::env::var("IMAP_USERNAME") {
        Ok(username) => username,
        Err(_) => {
            println!("Please set the IMAP_USERNAME environment variable");
            return;
        }
    };
    let password = match std::env::var("IMAP_PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            println!("Please set the IMAP_PASSWORD environment variable");
            return;
        }
    };

    let mut session = match client.login(username, password) {
        Ok(session) => session,
        Err(e) => {
            println!("Error logging in: {:?}", e);
            return;
        }
    };

    let caps = session.capabilities().unwrap();

    println!("Has move: {}", caps.has_str("MOVE"));


    for cap in caps.iter() {
        println!("{:?}", cap);
    }


}
