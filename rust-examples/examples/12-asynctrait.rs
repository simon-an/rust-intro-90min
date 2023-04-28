#[tokio::main]
async fn main() {
    // #[async_trait::async_trait]
    trait Foo {
        async fn foo(&self);
    }

    struct Bar;

    // #[async_trait::async_trait]
    impl Foo for Bar {
        async fn foo(&self) {
            println!("foo");
        }
    }

    let bar = Bar;
    bar.foo().await;

    let foo = &bar as &dyn Foo;
    foo.foo().await;
}
