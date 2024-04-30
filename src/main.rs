use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::process;

use ssh_localhost::SSHTunnel;

/// Host a localhost of a remote server to your local machine.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// SSH destination format: `[ADDRESS]` or `[USER]@[ADDRESS]` or `[USER]@[ADDRESS]:[SSH_PORT]`.  
    /// If you don't specify a ssh port, it will default to 22.
    destination: String,
    
    /// Remote server's localhost port.
    remote_port: u16,

    /// Local machine's port to host the remote server's localhost. Defaults to same as remote port.
    #[arg(required = false)]
    local_port: Option<u16>,
}

impl Args {
    fn to_ssh_tunnel(&self) -> SSHTunnel {
        let destination = self.destination.clone();
        let destination_parts: Vec<&str> = destination.split(':').collect();
        let destination = String::from(destination_parts[0]);
        let ssh_port = destination_parts.get(1).unwrap_or(&"22").parse::<u16>().unwrap();
        let remote_port = self.remote_port;
        let local_port = self.local_port.unwrap_or(remote_port);

        SSHTunnel {
            destination,
            ssh_port,
            remote_port,
            local_port,
        }
    }
}

fn main() {
    let args = Args::parse();
    let ssh_tunnel = args.to_ssh_tunnel();

    // connect
    match ssh_tunnel.start_tunnel() {
        Ok(_) => {
            println!("Localhost (remote server port: {}) is connected from {}.", ssh_tunnel.remote_port, ssh_tunnel.destination);
            println!("URL: http://localhost:{} (Press Ctrl+C to disconnect)", ssh_tunnel.local_port);
        },
        Err(_) => {
            eprintln!("Check if the SSH command is installed and the destination is correct.");
            process::exit(1);
        },
    }

    // wait for Ctrl+C to disconnect
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    while running.load(Ordering::SeqCst) {
    }

    // disconnect
    match ssh_tunnel.end_tunnel() {
        Ok(_) => println!("Disconnected successfully."),
        Err(e) => eprintln!("Error stopping tunnel: {}", e),
    }
}
