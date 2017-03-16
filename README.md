# Pseudo

### A small mocking library for Rust.

Pseudo lets you mock `Trait` implementations so that you can track function call arguments and set return values or overrides functions at test time.

Here's a quick example:

```rust
extern crate pseudo;

use pseudo::Mock;

trait Foo: Clone {
    fn expensive_fn(&self, x: i64, y: i64) -> i64;
}

#[derive(Clone)]
struct MockFoo {
    pub expensive_fn: Mock<(i64, i64), i64>,
}

impl Foo for MockFoo {
    fn expensive_fn(&self, x: i64, y: i64) -> i64 {
        self.expensive_fn.call((x + 10, y))
    }
}

fn double_expensive_fn<T: Foo>(foo: &T, x: i64, y: i64) -> i64 {
    foo.expensive_fn(x, y) * 2
}

#[test]
fn doubles_return_value() {
    let mock = MockFoo { expensive_fn: Mock::default() };

    mock.expensive_fn.return_value(1000);

    assert_eq!(double_expensive_fn(&mock, 1, 2), 2000);
}

#[test]
fn uses_correct_args() {
    let mock = MockFoo { expensive_fn: Mock::default() };

    assert!(!mock.expensive_fn.called());

    double_expensive_fn(&mock, 1, 2);

    assert_eq!(mock.expensive_fn.num_calls(), 1);
    assert!(mock.expensive_fn.called_with((11, 2)));
}
```

More examples are available in the [examples directory](./examples).
