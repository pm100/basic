# Basic Rust Project

This is a basic Rust project created to demonstrate the structure and setup of a Rust application.

## Project Structure

```
basic
├── src
│   └── main.rs
├── Cargo.toml
└── README.md
```

## Getting Started

To get started with this project, ensure you have Rust and Cargo installed on your machine. You can download them from [the official Rust website](https://www.rust-lang.org/).

## Building the Project

To build the project, navigate to the project directory and run:

```
cargo build
```

## Running the Project

To run the project, use the following command:

```
cargo run
```

## Logging

The interpreter now initializes a standard Rust logger with `env_logger`. By default it logs warnings and errors. To see `dyncall`'s internal trace output, set `RUST_LOG` when launching the interpreter:

```powershell
$env:RUST_LOG = "dyncall=trace"
cargo run -- test_dyncall_output_num.bas
```

## Contributing

Feel free to contribute to this project by submitting issues or pull requests. 

## License

This project is licensed under the MIT License.
