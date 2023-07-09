mod io;
mod logic;
mod model;

use logic::{get_available_hotels, get_cheapest_hotels_for_input};

fn main() {
    let hotels = get_available_hotels();
    let results = get_cheapest_hotels_for_input();
    println!("{:?}", results);
}

/*
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio;

// What we need to know is that a future is what you give an async runtime.
// You do this by .awaiting it.
// Then the runtime gives you back the result.

enum Hello {
    Init { name: &'static str },
    Done,
}

impl Future for Hello {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // do something depending on the current state
        match *self {
            Hello::Init { name } => println!("hello, {name}!"),
            Hello::Done => panic!("Please stop polling me!"),
        };
        // transition state
        *self = Hello::Done;
        // signal it is ready
        Poll::Ready(())
    }
}

// Because we're returning a "hand made" future, we can't use the async keyword.
fn hello(name: &'static str) -> impl Future<Output = ()> {
    Hello::Init { name }
}

struct YieldNow {
    yielded: bool,
}
impl Future for YieldNow {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {

        // yield means returning control to the async runtime
        println!("YieldNow: poll()");
        if self.yielded == true {
            return Poll::Ready(());
        }

        self.yielded = true;

        cx.waker().wake_by_ref();

        Poll::Pending
    }
}

fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

// async fn hello(name: &'static str) {
//     println!("hello, {name}!");
// }

fn main() {
    let body = async {
        println!("Before yield_now().await");
        yield_now().await;
        println!("After yield_now().await");
    };

    return tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body);
}

// #[tokio::main]
// async fn main() {
//     // What happens behind the .await syntax is that the poll function gets called.
//     //hello("world").await;
//     yield_now().await
// }

// fn main() {
//     // let hotels = get_available_hotels();
//     // let bests = best_price_hotel(hotels);

//     // println!("{:?}", bests);
// }

*/
