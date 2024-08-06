# c4

Rust C-like for-loop macro.

```rust
fn main() {
    c4! {
        for (let mut i = 1; i <= 10; i += 1) {
            println!("9 * {:<2} = {}{}", i, i - 1, 10 - i);
        }
    }
    c4! {
        for (
            let (mut i, mut j) = (0, 0),
            let mut s = "some dummy word".to_string();
            i * j <= s.len();
            i += 1,
            s = format!("{} {} {}", &s, i, j)
        ) {
            j += 1;
            println!("i: {}; j: {}; s: {}", i, j, s);
        }
    }
}
```
