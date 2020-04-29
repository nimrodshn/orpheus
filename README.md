# orpheus
An experimental key-value storage engine written in rust.

## Architecture
`orpheus` is a key value store. It uses an [SSTable](https://en.wikipedia.org/wiki/Log-structured_merge-tree) (in-memory sorted table) for indexing and a persistent log containes the raw bytes of given for values;
Each value is appended as UTF encoded byte array to the log file and its key, offset and length are 
stored in an in memory hashmap for later retreival.

## How to run ?
Compile using `cargo build` and run providing the relevant flags:
```
cargo run -- --log-path=/path/to/log/file.txt --port=<port>
```

## How to use ?
`orpehus` runs a small server which allowes the user to interact with the engine, for example:
```
// Write key-value pair to the database
curl --header "Content-Type: application/json" -X POST --data '{"key":"hello", "value":"world"}' http://localhost:8000
// Retrieve value by a given key
curl --header "Content-Type: application/json" -X GET --data '{"key":"hello"}' http://localhost:8000
{"key":"hello","value":"world"}
```

## License
MIT.