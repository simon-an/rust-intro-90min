fn main() {
    
    trait Foo {
        fn foo(&self);
    }

    struct Bar;

    impl Foo for Bar {
        fn foo(&self) {
            println!("foo");
        }
    }

    let bar = Bar;
    bar.foo();

    let foo = &bar as &dyn Foo;
    foo.foo();

}
