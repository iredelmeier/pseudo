// Based on https://github.com/iredelmeier/pseudo/issues/1
extern crate pseudo;

use pseudo::Mock;

pub trait A {
    fn foo(&self);
}

struct MockA {
    foo: Mock<(), ()>,
}

impl A for MockA {
    fn foo(&self) {
        self.foo.call(())
    }
}

fn main() {
    let mock = MockA {
        foo: Mock::default(),
    };
    let _ = Box::new(mock) as Box<A + Send>;
}
