struct A {
}

impl A {
    fn b() -> B {
        B {
        }
    }
}

struct Ab {
}

impl Ab {
    fn b() -> B {
        B {
        }
    }
}

struct B {
}

impl B {
    fn a() -> Ab {
        Ab {
        }
    }
}