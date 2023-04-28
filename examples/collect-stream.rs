use async_stream::stream;
use futures::stream::StreamExt;
use futures_core::stream::Stream;

fn zero_to_three() -> impl Stream<Item = u32> {
    stream! {
        for i in 0..3 {
            yield i;
        }
    }
}

fn double<S: Stream<Item = u32>>(input: S) -> impl Stream<Item = u32> {
    stream! {
        for await value in input {
            yield value * 2;
        }
    }
}

#[tokio::main]
async fn main() {
    let s = double(zero_to_three());
    assert_eq!(vec![0, 2, 4], s.collect::<Vec<_>>().await);
}
