# Clacks

## Local development

### Running

You need Node v25, Yarn and the Rust toolchain installed on your machine.

Run the following in the first terminal:

    $ cd clacks-backend
    $ cargo run -- run local_config.toml

Run the following in the second terminal:

    $ cd clacks-frontend
    $ yarn serve

Next, open http://localhost:8080 in your browser.

### Creating a pull request

Make sure that the backend CI pipeline passes locally:

    $ cd clacks-backend
    $ make ci

Make sure that the frontend linter doesn't return any errors:

    $ cd clacks-frontend
    $ yarn lint

## Building

To build a self-contained and ready-to-deploy binary for the Raspberry Pi you
need `cargo` as well as Docker or Podman installed locally.

### Building using Podman

    $ CONTAINER_ENGINE=podman make

### Building using Docker

    $ CONTAINER_ENGINE=docker make 

## Hardware

- Raspberry PI Zero
- Raspberry PI expansion board with PCA9685
- Servos: 6x PowerHD HD-1160A
- Lights: 6x LED RGB WS2812B 5050 x 16 LEDs 68mm ring
- Power supply: 12V/5A
