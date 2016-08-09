# future-fetch
> http GET implemented with the new Rust Futures lib

## usage

```rust
extern crate future_fetch;

fn main() {
  let data = future_fetch::fetch("www.rust-lang.org", "text/plain");
  println!("{}", data);
}
```
