# rgrep
A Rust inspired implementation of GNU Grep.

This should not be considered for production use at this time as it was merely a learning project for Rust for fun.

Use at your own risk!

### Install Locally

Clone the repo down and then run these commands to install the binary locally.
```shell
cargo install --path . --locked
```

Open a new terminal and run the following command to see the help menu.
```shell
rgrep --help
```

### Usage

```shell
rgrep <path> <pattern>
```

EXAMPLE:
```shell
rgrep ./names.csv "john doe"
```
