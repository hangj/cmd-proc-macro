# cmd-proc-macro

This crate contains only 1 proc-macro `cmd_execute` that can execute shell commands and yield an expression of type `&'static [u8; N]` which is the output of the commands

# Usage

src/main.rs:

```rust
let cargo = cmd_execute!("cat Cargo.toml");
let bytes = include_bytes!("../Cargo.toml");
assert_eq!(cargo, bytes);
```

