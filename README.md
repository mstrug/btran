# CSV transactions processor application

## Usage

Run application with specyfing `csv` file as arument:
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

## Test

To run unit tests use following command: `cargo test`

## Other info

Application developed on Debian, rustc version `.63.0.
