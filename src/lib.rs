// Pseudo is a small library for making mock `Trait` implementations.
//
// The `Mock` struct tracks function call arguments and specifies return
// values or function overrides.
//
// # Examples
//
// Using `Mock` for testing:
//
// ```
// use subtitute::Mock;
//
// trait Foo {
//     fn test_fn(&self, arg: i64) -> i64;
// }
//
// struct MockFoo {
//     pub test_fn: Mock<i64, i64>,
// }
//
// impl Foo for MockFoo {
//     fn test_fn(&self, arg: i64) -> i64 {
//         self.test_fn.call(arg)
//     }
// }
//
// impl Default for MockFoo {
//     fn default() -> Self {
//         MockFoo { test_fn: Mock::default() }
//     }
// }
//
// fn fn_using_foo_trait<T: Foo>(x: i64, y: i64, foo: T) -> i64 {
//     foo.test_fn(x + y)
// }
//
// struct StructUsingFoo<T: Foo> {
//     foo: T,
// }
//
// impl<T: Foo> StructUsingFoo
//
// let mock = MockFoo::default();
//
// ```
//
// Using `Mock` for functions with various signature types:
//
// ```
// use pseudo::Mock;
//
// trait Foo {
//     fn simple_fn(&self, arg: String) -> String;
//     fn void_fn(&self, arg: String);
//     fn zero_arg_fn(&self) -> String;
//     fn multi_arg_fn(&self, x: i64, y: bool) -> bool;
// }
//
// struct MockFoo {
//     pub simple_fn: Mock<String, String>,
//     pub void_fn: Mock<String, ()>,
//     pub zero_arg_fn: Mock<(), String>,
//     pub multi_arg_fn: Mock<(i64, bool), bool>
// }
//
// impl Default for MockFoo {
//     fn default() -> Self {
//         MockFoo {
//             simple_fn: Mock::default(),
//             void_fn: Mock::default(),
//             zero_arg_fn: Mock::default(),
//             multi_arg_fn: Mock::default(),
//         }
//     }
// }
//
// impl Foo for MockFoo {
//     fn simple_fn(&self, arg: String) -> String {
//         self.simple_fn.call(arg)
//     }
//
//     fn void_fn(&self, arg: String) {
//         self.void_fn.call(arg)
//     }
//
//     fn zero_arg_fn(&self) -> String {
//         self.zero_arg_fn.call(())
//     }
//
//     fn multi_arg_fn(&self, x: i64, y: bool) -> bool {
//         self.multi_arg_fn.call((x, y))
//     }
// }
// ```
pub use mock::Mock;

mod mock;
