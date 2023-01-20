## General
____________

### Author
* Josh McIntyre

### Website
* jmcintyre.net

### Overview
* RustyAuto is a simple OBD code search tool

## Development
________________

### Git Workflow
* master for releases (merge development)
* development for bugfixes and new features

### Building
* `make` - wraps the cargo build tool
* `make run` - wraps the cargo run tool
* `cargo build` - build the application
* `cargo run` - build and run the application

### Features
* Pick database - OBDII codes, OBI for Ford, GM, Toyota

### Requirements
* Requires Rust language build tools
* Requires text file databases of OBD codes

### Platforms
* Windows
* MacOSX
* Linux

## Usage
____________

### Command Line Usage
* Put the formatted data file in the same directory as the compiled binary
* Run `cargo run`
* Pick the database
* Enter the desired OBD code