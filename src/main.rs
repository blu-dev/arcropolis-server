use std::io::Write;

fn receive(message: String) {
    println!("{}", message);
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let ip_addr = match args.next() {
        Some(str) => str,
        None => {
            eprintln!("Usage: arcropolis-server <ip> <port>");
            return;
        }
    };
    let port = match args.next() {
        Some(str) => {
            match str.parse::<u16>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Usage: arcropolis-server <ip> <port>");
                    return;
                }
            }
        },
        None => {
            eprintln!("Usage: arcropolis-server <ip> <port>");
            return;
        }
    };
    let _ = std::thread::spawn(move || {
        skyline_communicate::set_on_receive(skyline_communicate::Receiver::Normal(receive));
        skyline_communicate::start_client(ip_addr.as_str(), port);
    });

    loop {
        if !skyline_communicate::is_connected() {
            std::thread::sleep(std::time::Duration::from_millis(100));
            continue;
        }
        use std::io::{stdout, stdin};
        let mut s = String::new();
        std::thread::sleep(std::time::Duration::from_millis(500));
        print!("client > ");
        stdout().flush().unwrap();
        stdin().read_line(&mut s).expect("Failed to read from standard input.");
        skyline_communicate::send(s.as_str());
    }
}