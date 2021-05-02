use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

#[tokio::main]
async fn main() {
    println!("Hello.");

    greet_async().await;
    greet_async_without_async_prefix().await;
    FutureImpl {}.await.unwrap();
    get_future_impl().await;
    Greeter {}.run().await.unwrap();
}

async fn greet_async() {
    println!("Hello from greet_async function.");
}

fn greet_async_without_async_prefix() -> impl Future {
    async move {
        greet_async().await;
        println!("But without async prefix.");
    }
}

struct FutureImpl {}

impl Future for FutureImpl {
    type Output = Result<(), Box<dyn Error>>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        Poll::Ready(Ok(()))
    }
}

fn get_future_impl() -> impl Future {
    FutureImpl {}
}

trait Greet {
    type Future: Future<Output = Result<(), Box<dyn Error>>>;
    fn run(&self) -> Self::Future;
}

struct Greeter {}

impl Greet for Greeter {
    type Future = Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>>;

    fn run(&self) -> Self::Future {
        Box::pin(async move {
            println!("Hello from Greeter.");
            Ok(())
        })
    }
}
