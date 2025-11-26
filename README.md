# Clacks

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
