# CSV transactions processor application

## Usage

Run application specyfing `csv` file as an argument:
```
btran <csv input file>
```
or
```
cargo run -- <csv input file>
```

Application outputs to `stdout`.

## CSV file input format

Columns: type, client, tx, amount

type values: deposit, withdrawal, dispute, resolve, chargeback

client: client ID

tx: transaction ID (unique)

amount: transaction value (only for deposit and withdrawal)

## Test

To run unit tests use following command: `cargo test`

## Other info

Application developed on Debian, rustc version 1.63.0.
