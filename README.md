# #[todo]
A simple proc-macro for (forward-) declaring unimplemented functions.

## Usage
```rust
use todop::todo;

pub struct MyStruct;

impl MyStruct {
    #[todo]
    pub fn new() -> Self;
    
    #[todo]
    pub fn my_method(arg1: &str, arg2: f32) -> Option<String> {
        let a = format!("{arg1} -> {arg2}");
        // Not finished
    }
}

#[todo]
fn my_function();
```
