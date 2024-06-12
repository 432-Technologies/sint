# System Independent Timer

Async timer that works on wasm (with gloo) and linux/macos/windows (with tokio)

```rust
async {
    sint::Timeout::new(sint::Duration::from_millis(500)).await;

    //Or equivalent
    sint::sleep(Duration::from_millis(500)).await;
};
```
