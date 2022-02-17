# n-times

## Overview

This crate provides `Countable` trait that makes possible using unsigned integer values with pointer size (`usize`) with method `times()` which gives possibility to repeat concrete operation *n* number of times. 

## Getting Started

Add following lines to `Cargo.toml` file to use crate:

```toml
[dependencies]
n-times = "0.1"
```

Additionally, you have to import the `Countable` trait to use it. 

```rust
use n_times::Countable;
```

## Example usage

Simple example that explaining idea of usage:

```rust
use n_times::Countable;

fn main() {
    let amount_of_something = 10usize;

    amount_of_something.times().run(|| println!("Hello World!"));
}
```

Output:
```
Hello World!
Hello World!
Hello World!
Hello World!
Hello World!
```

Additionally, is it possible to directly implement Countable trait to external structures:

```rust
use n_times::Countable;

struct Foo;

impl Into<usize> for Foo {
    fn into(self) -> usize {
        5usize
    }
}

fn main() {
    let amount_of_something = Foo;

    amount_of_something.times().run(|| println!("Hello World!"));
}
```

Output:

```
Hello World!
Hello World!
Hello World!
Hello World!
Hello World!
```


## But... why?

The inspiration to this project can be found in the standard library, where there is possibility for `bool` type to use functional style conditional statement:

```rust
fn main() {
    let some_condition = true;
    
    some_condition.then(|| println!("Hello World!"));
}
```

This syntax looks (in my opinion) much better than standard one. So why not make our codes much more readable and make less indentations in the code?

```rust
use n_times::Countable;

fn main() {
    let some_condition = true;
    let amount_of_something = 10usize;
    
    some_condition.then(|| amount_of_something.times().run(|| println!("Hello World!")));
}
```