# orpheus
An experimental key-value store written in rust.

## Architecture
`orpheus` is a key value store. It uses a simple log file and an in memory table as storage for the DB;
Each value is appended as UTF encoded byte array to the log file and its key, offset and length are 
stored in an in memory hashmap for later retreival.

## How to run ?
Compile using `cargo build` and run providing the relevant flags:
```
cargo run -- --log-path=/path/to/log/file.txt --port=<port>
```

## License
MIT.