# MySK API

MySK API is a monolithic backend which acts as an API server. It is the middleman between the
frontend deployment and the MySK database.

## Issues and Requests

If you have any issues or requests, file an issue
[here](https://github.com/suankularb-wittayalai-school/mysk-api/issues). Please look through the
existing issues before submitting a new one. Both Thai and English are welcome.

## Development

### Commands

Run these while at the project root.

| Command        | Description                                                |
| -------------- | ---------------------------------------------------------- |
| `cargo fmt`    | Formats the entire project with `rustfmt`.                 |
| `cargo clippy` | Lints the code with `clippy`. Required before pushing.     |
| `cargo run`    | Compiles and runs MySK API, defaults to running in debug.  |
| `cargo build`  | Compiles MySK API without running it.                      |

### Directories

| Directory                       | Description                               |
| ------------------------------- | ----------------------------------------- |
| `mysk-data-api/`                | The main codebase of MySK API.            |
| `mysk-lib/`                     | The library that MySK API utilises.       |
| `mysk-lib-derives/`             | Derive macros to ease development.        |
| `mysk-lib-macros/`              | Procedural macros to ease development.    |
