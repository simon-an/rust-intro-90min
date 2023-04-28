use async_stream::stream;

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

fn zero_to_three() -> impl Stream<Item = u32> {
    stream! {
        for i in 0..3 {
            yield i;
        }
    }
}

// #[async_trait::async_trait]
// trait MyS {
//     async  fn foo(&self);
//     fn zero_to_three(&self) -> impl Stream<Item = u32>;
// }
// struct S {

// }
// #[async_trait::async_trait]
// impl MyS for S{
//     async fn foo(&self){}
//     fn zero_to_three(&self) -> impl Stream<Item = u32> {
//         stream! {
//             for i in 0..3 {
//                 yield i;
//             }
//         }
//     }
// }

fn double<S: Stream<Item = u32>>(input: S) -> impl Stream<Item = u32> {
    stream! {
        for await value in input {
            yield value * 2;
        }
    }
}

#[tokio::main]
async fn main() {
    // let s1 = S{}.zero_to_three();

    let s = double(zero_to_three());
    pin_mut!(s); // needed for iteration

    while let Some(value) = s.next().await {
        println!("got {}", value);
    }
}
