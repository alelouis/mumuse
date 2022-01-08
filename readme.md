# mumuse

Small midi parser and music theory library written in Rust *(wip)*.  
Scroll documentation : [**doc.rs/mumuse**](https://docs.rs/mumuse/0.1.0/mumuse/)
```rust 
fn main() {
    midi::show_input_ports();
    midi::receive("Arturia KeyStep 32".to_string());
}
```

<p align="center">
  <img width="1000" src="capture.png">
</p>