# ScyllaDB Crypto Ticker

This is a demo of ScyllaDB with [Binance websocket market streams](https://binance-docs.github.io/apidocs/spot/en/#websocket-market-streams). 
It is a simple binary that charts live prices at 2 second resolution for a given pair.

## Example Output

    cargo run -p scylla-crypto-ticker eth usdt

![img.png](img.png)

## Usage

To build:

    make build

To create database

    make up

To migrate database

    make migrate

To run the demo

    make run

To reset the database

    make reset

To destroy the database

    make down

## Dependencies

* [ScyllaDB](https://www.scylladb.com/)
* [Rust](https://www.rust-lang.org/)
* [Docker](https://www.docker.com/)
* [Docker Compose](https://docs.docker.com/compose/)
* [GNU Make](https://www.gnu.org/software/make/)
