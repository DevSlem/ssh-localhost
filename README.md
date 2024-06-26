# SSH-Localhost

You can host a localhost of a remote server on your local machine using SSH. This is useful when you host a localhost on a remote server and you want to access it on your local machine. For example, when you execute [Tensorboard](https://www.tensorflow.org/tensorboard) on the remote server where you're training a model, it is hosted on the remote server's localhost. If you use SSH command, you can access it on your local machine and stop the connection when you don't need to access it anymore. But, this process is too complicated using SSH command. So, I implemented a simple script to do this.

## Installation

If you have [Rust](https://www.rust-lang.org/) installed on your machine, you can install the script using the following command:

```bash
cargo install --git https://github.com/DevSlem/ssh-localhost.git
```

If you don't have Rust installed, you can download the binary at [Releases](https://github.com/DevSlem/ssh-localhost/releases) page and put it in your PATH.

## Usage

Let's assume that you're executing Tensorboard on the remote server where the user name is `testuser` and the IP address is `123.456.789.0`. The remote server's ssh port is open on `1234`. Tensorboard is hosted on the remote server's localhost port `6006` (Tensorboard default port). You want to access it on your local machine using port `4004`. In this case, you can use the following command:

```bash
ssh-localhost -p 1234 testuser@123.456.789.0 6006 4004
```

Then, you can access it at the url `http://localhost:4004` on your local machine.

Below is the help message of the script:

```
Host a localhost of a remote server to your local machine

Usage: ssh-localhost [OPTIONS] <DESTINATION> <REMOTE_PORT> [LOCAL_PORT]

Arguments:
  <DESTINATION>  SSH destination format: `[ADDRESS]` or `[USER]@[ADDRESS]` or `[USER]@[ADDRESS]:[SSH_PORT]`. If you don't specify a ssh port, it will default to 22
  <REMOTE_PORT>  Remote server's localhost port
  [LOCAL_PORT]   Local machine's port to host the remote server's localhost. Defaults to same as remote port

Options:
  -p <P>         SSH port of the remote server. Defaults to one specified in SSH config
  -h, --help     Print help
  -V, --version  Print version
```

> Note that If you don't specify `[LOCAL_PORT]`, it will default to `[REMOTE_PORT]`. For example, `ssh-localhost -p 1234 testuser@123.456.789.0 6006` is equivalent to `ssh-localhost -p 1234 testuser@123.456.789.0 6006 6006`.

You can simply stop the connection by pressing `Ctrl+C`.

### SSH Config

If you already configured your SSH config file (e.g., `~/.ssh/config`) like this:

```
Host testserver
  HostName 123.456.789.0
  User testuser
  Port 1234
```

then, you can easily access the remote server's localhost using the following simple command:

```bash
ssh-localhost testserver 6006
```

In this case, you don't need to specify user name, IP address, and SSH port.