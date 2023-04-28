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

fn double<S: Stream<Item = u32>>(input: S) -> impl Stream<Item = Result<u32, &'static str>>
// -> impl Stream<Item = anyhow::Result<u32>>
{
    stream! {
        for await value in input {
            if value == 0 {
                // yield Err(anyhow::anyhow!("bla"));
                yield Err("bla");
            } else {
                yield Ok(value * 2);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let s = double(zero_to_three());
    assert_eq!(
        vec![Err("bla"), Ok(2u32), Ok(4u32)],
        s.collect::<Vec<_>>().await
    );
    // assert_eq!(vec![Err(anyhow::anyhow!("bla")), Ok(2u32), Ok(4u32)], s.collect::<Vec<_>>().await);
}
