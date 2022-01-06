# midi-parse

Small midi parser written in Rust for debug purposes.

```rust 
fn main() {
    midi::show_input_ports();
    midi::receive("Arturia KeyStep 32".to_string());
}
```

<p align="center">
  <img width="1000" src="capture.png">
</p>