use std::convert::{TryFrom, TryInto};

fn main() {
    #[derive(Debug)]
    struct Foo {
        a: i32,
    }

    #[derive(Debug)]
    struct Bar {
        a: String,
    }

    impl From<Foo> for Bar {
        fn from(f: Foo) -> Self {
            Bar { a: f.a.to_string() }
        }
    }

    impl TryFrom<Bar> for Foo {
        type Error = std::num::ParseIntError;
        fn try_from(value: Bar) -> Result<Self, Self::Error> {
            Ok(Foo { a: value.a.parse()? })
        }
    }

    let bar = Bar {
        a: "123".to_string(),
    };

    let foo: Foo = bar.try_into().expect("failed to convert");
    println!("{:?}", foo);

    let foo = Foo {
        a: 1234,
    };

    let bar: Bar = foo.into();
    println!("{:?}", bar)
}
