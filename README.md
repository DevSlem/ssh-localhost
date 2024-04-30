# SSH-Localhost

You can host a localhost of a remote server on your local machine using SSH. This is useful when you host a localhost on a remote server and you want to access it on your local machine. For example, when you execute [Tensorboard](https://www.tensorflow.org/tensorboard) on the remote server where you're training a model, it is hosted on the remote server's localhost. If you use SSH command, you can access it on your local machine and stop the connection when you don't need to access it anymore. But, this process is too complicated using SSH command. So, I implemented a simple script to do this.

## Installation

If you have [Rust](https://www.rust-lang.org/) installed on your machine, you can install the script using the following command:

```bash
cargo install --git https://github.com/DevSlem/ssh-localhost.git
```

If you don't have Rust installed, you can download the binary at [Releases](https://github.com/DevSlem/ssh-localhost/releases) page and put it in your PATH.

## Usage

Let's assume that you're executing Tensorboard on the remote server where the user name is `testuser` and the IP address is `123.456.789.0`. The remote server's ssh port is open on `1234`. Tensorboard is hosted on the remote server's localhost port `6006` (default). You want to access it on your local machine using port `4004`. You can use the script as follows:

```bash
ssh-localhost testuser@123.456.789.0:1234 6006 4004
```

Then, you can access it at the url `http://localhost:4004` on your local machine.

Below is the help message of the script:

```
Host a localhost of a remote server to your local machine

Usage: ssh-localhost <DESTINATION> <REMOTE_PORT> [LOCAL_PORT]

Arguments:
  <DESTINATION>  SSH destination format: `[ADDRESS]` or `[USER]@[ADDRESS]` or `[USER]@[ADDRESS]:[SSH_PORT]`. If you don't specify a ssh port, it will default to 22
  <REMOTE_PORT>  Remote server's localhost port
  [LOCAL_PORT]   Local machine's port to host the remote server's localhost. Defaults to same as remote port

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Note that If you don't specify `[LOCAL_PORT]`, it will default to `[REMOTE_PORT]`. For example, `ssh-localhost testuser@123.456.789.0:1234 6006` is equivalent to `ssh-localhost testuser@123.456.789.0:1234 6006 6006`.
