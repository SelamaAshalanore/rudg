use std::fmt::Debug;

#[derive(Debug)]
struct A<T> where T: Debug {
    a: T,
}

impl<T> A<T> where T: Debug {
    fn a(a: T) -> Self {
        A {
            a: a,
        }
    }
}

impl <T>B<T> for A<T> where T: Debug {
    fn a(&self) -> Option<T> {
        None
    }
}

trait B<T> : Debug where T: Debug {
    fn a(&self) -> Option<T>;
}

impl <T>B<T> {
    fn a(&self) -> Option<T> {
        None
    }
}