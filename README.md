## Retry


```rust
use retry::prelude::*;
#[tokio::main]
pub async fn main() {
    let arg1 = 100
    foo.retry::<3>(arg1).await;
}
pub async fn foo(arg1: u32) -> Result<(), ()> {
    // do_stuff().await
}
```
