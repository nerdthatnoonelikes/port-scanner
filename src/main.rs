use std::net::TcpStream;
use std::io::{stdin, stdout, Write};
use std::format;
fn input(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    let mut ipaddr = String::new();

    let mut port = String::new();

    print!("Please input an ipadress: ");
    input(&mut ipaddr);

    print!("Please input a port: ");
    input(&mut port);

    let ipaddr = ipaddr.trim();
    let port = port.trim();

    match TcpStream::connect(format!("{}:{}", ipaddr, port)) {
        Ok(_) => {
            println!("It appears there is life")
        }

        Err(_) => {
            println!("It appears there is no life")
        }
    }
}
