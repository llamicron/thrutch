# Thrutch
Connect to servers

**Why?**

I was tired of remembering usernames and IP addresses for my raspbery pis. Previously, I would make an alias in my `~/.bash_profile` but that got hard to manage. This is an easier way.

## Install
**See the latest release** ([1.0.0](https://github.com/llamicron/thrutch/releases/tag/1.0.0)) for binaries. Download the `thrutch` binary and put in somewhere in your path, like `~/bin/` or `/usr/local/bin`.

*or*

**If you want the latest version**, clone this repo and compile it with `cargo`
```
$ git clone https://github.com/llamicron/thrutch/
$ cd thrutch
$ cargo build --release
$ cp target/release/thrutch /usr/local/bin
$ thrutch
```

## Usage
These are the current commands:
```
$ thrutch list               # list all servers
$ thrutch add                # add a new server
$ thrutch remove             # remove a server
$ thrutch connect [server]   # connect to a server

$ thrutch                # see the help page
$ thrutch --version      # see the current version
```

Thrutch will ask for any input needed. Thrutch runs `ssh` to connect to a server. It does not manage passwords or private keys. The main use of this tool is to store IP address and usernames so you don't need to remember them. Useful if you have lots of remote servers like reaspberry pis.


## License and Contributing
MIT License, see `LICENSE` for more info. If you'd like to contribute, just fork, develop, then open a pull request on Github.
