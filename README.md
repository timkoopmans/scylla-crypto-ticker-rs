# ScyllaDB demo with Binance L2 Order Books

This is a demo of ScyllaDB with Binance L2 Order Books. 
It is a simple binary that charts bid or ask price depth for a given pair.

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

## Example Output

    cargo run -p scylladb-order-book-rs btc usdt

![img.png](img.png)
