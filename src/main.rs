#![deny(clippy::all)]

use std::{io::Read, os::unix::net::UnixListener};

fn main() -> std::io::Result<()> {
    println!("service is running ...");
    // let listener = UnixListener::bind("/var/run/docker.sock")?;
    let listener = UnixListener::bind("/src/var/sock/docker.sock")?;

    match listener.accept() {
        Ok((mut socket, addr)) => {
            println!("Got a client: {:?} - {:?}", socket, addr);
            let mut response = String::new();
            socket.read_to_string(&mut response)?;
            println!("{}", response);
        }
        Err(e) => println!("accept function failed: {:?}", e),
    }

    Ok(())
}
