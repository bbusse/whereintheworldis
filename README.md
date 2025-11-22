# wintwi

A minimal CLI tool to geolocate IP addresses using [ip-api.com](http://ip-api.com/).

## Description
`wintwi` queries country, region, city, and coordinates for a given IP address (or your own if none is provided).


## Usage

```sh
./wintwi [--ip-address <IP>] [--output <field>]
```
- `--ip-address <IP>`: Optional. IP address to geolocate. If omitted, uses your own IP.
- `--output <field>`: Optional. Print only the specified field. Supported fields:
  - `ip`, `country`, `region`, `city`, `latitude`, `longitude`, `coordinates`

### Examples

Show all info for your own IP:
```sh
./wintwi
```

Show only the city for a specific IP:
```sh
./wintwi --ip-address 8.8.8.8 --output city
```

Show coordinates for a specific IP:
```sh
./wintwi --ip-address 8.8.8.8 --output coordinates
```

## Installation

### Install directly from git
You can install the binary using cargo:
```sh
cargo install --git https://github.com/bjoernb/whereintheworldis --bin wintwi
```

### Prerequisites
- Rust toolchain (https://rustup.rs)

### Clone the repository
```sh
git clone https://github.com/bjoernb/whereintheworldis.git
cd whereintheworldis
```

### Build
```sh
cargo build --release
```
Or use make:
```sh
make
```

## License
BSD-3-Clause

Author: Björn Busse <bj.rn@baerlin.eu>
