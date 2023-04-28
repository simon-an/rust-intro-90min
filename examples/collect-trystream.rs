use async_stream::{stream, try_stream};
use futures::stream::StreamExt;
use futures_core::stream::Stream;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
#[error("Bla {0}")]
struct StreamError(String);

fn zero_to_three() -> impl Stream<Item = u32> {
    stream! {
        for i in 0..3 {
            yield i;
        }
    }
}

fn double<S: Stream<Item = u32>>(input: S) -> impl Stream<Item = Result<u32, StreamError>> {
    try_stream! {
        for await value in input {
            if value == 1 {
                Err(StreamError("stream failed".to_string()))?;
            } else {
                yield value * 2;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let s = double(zero_to_three());

    assert_eq!(
        vec![Ok(0u32), Err(StreamError("stream failed".to_string()))],
        s.collect::<Vec<_>>().await
    );
}
