use std::process::{self, Command, Stdio};
use std::io::{self, ErrorKind};

pub struct SSHTunnel {
    pub destination: String,
    pub ssh_port: u16,
    pub remote_port: u16,
    pub local_port: u16,
}

impl SSHTunnel {
    pub fn start_tunnel(&self) -> Result<(), std::io::Error> {
        let command = format!(
            "ssh -NfL localhost:{}:localhost:{} {} -p {}",
            self.local_port, self.remote_port, self.destination, self.ssh_port
        );

        // Execute the command and capture any potential errors
        let child = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::piped())
            .spawn();

        match child {
            Ok(mut child) => {
                // We use `wait` here to get the exit status of the ssh command
                let result = child.wait()?;
                if result.success() {
                    Ok(())
                } else {
                    Err(io::Error::new(ErrorKind::Other, "SSH command failed to start correctly"))
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn end_tunnel(&self) -> Result<(), std::io::Error> {
        let command = format!(
            "kill $(lsof -t -i:{})",
            self.local_port
        );

        process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()?
            .wait()?;

        Ok(())
    }
}
