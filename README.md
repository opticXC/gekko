# Gekko (Name WIP)

A template repository for a [Rocket.rs](https://rocket.rs/) + [React](https://reactjs.org/) web application.

## Getting Started

Please Fork the repository and clone it to your local machine before proceeding.

### Prerequisites
> - [Rust](https://www.rust-lang.org/tools/install)
> - A JS Bundler (e.g. [Bunjs](https://bun.sh/))

### Instructions
> 1. Install the Rust dependencies:
> run `cargo fetch` in the root directory.
> 2. Install the JS dependencies:
> run `npm/yarn/bun install` in the `web` directory.
> 3. Build the JS bundle:
> run `npm/yarn/bun run compile` in the `web` directory.
> 4. Run the application:
> run `cargo run` in the root directory.

for release builds, run the `scripts/build-web` script in the root directory.
and then run the `cargo run --release` command in the root directory.

## License
MIT 
