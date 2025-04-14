# OneLog

## Debug

```
RUST_LOG=debug cargo run "Hello onelog info log."
```

LogType

```
RUST_LOG=debug cargo run "This is a warning" -t warn
```

```
RUST_LOG=debug cargo run "Something went wrong" -t error
```

```
RUST_LOG=debug cargo run "Debug details" -t debug
```
