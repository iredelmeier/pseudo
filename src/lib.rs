//! Pseudo is a small library for mocking `Trait` implementations.
//!
//! The `Mock` struct tracks function call arguments and specifies return
//! values or function overrides.
//!
//! # Examples
//!
//! ```
//! use pseudo::Mock;
//!
//! trait Foo: Clone {
//!     fn expensive_fn(&self, x: i64, y: i64) -> i64;
//! }
//!
//! #[derive(Clone)]
//! struct MockFoo {
//!     pub expensive_fn: Mock<(i64, i64), i64>,
//! }
//!
//! impl Foo for MockFoo {
//!     fn expensive_fn(&self, x: i64, y: i64) -> i64 {
//!         self.expensive_fn.call((x + 10, y))
//!     }
//! }
//!
//! fn double_expensive_fn<T: Foo>(foo: &T, x: i64, y: i64) -> i64 {
//!     foo.expensive_fn(x, y) * 2
//! }
//!
//! fn test_doubles_return_value() {
//!     let mock = MockFoo { expensive_fn: Mock::default() };
//!
//!     mock.expensive_fn.return_value(1000);
//!
//!     assert_eq!(double_expensive_fn(&mock, 1, 2), 2000);
//! }
//!
//! fn test_uses_correct_args() {
//!     let mock = MockFoo { expensive_fn: Mock::default() };
//!
//!     assert!(!mock.expensive_fn.called());
//!
//!     double_expensive_fn(&mock, 1, 2);
//!
//!     assert_eq!(mock.expensive_fn.num_calls(), 1);
//!     assert!(mock.expensive_fn.called_with((11, 2)));
//! }
//!
//! test_doubles_return_value();
//! test_uses_correct_args();
//! ```

pub use mock::Mock;

pub type Pseudo<C, R> = Mock<C, R>;

mod mock;
