extern crate actix_web;
extern crate futures;
extern crate tokio_timer;

use std::time::{Duration, Instant};

use actix_web::{actix, client};
use actix_web::actix::Actor;

use futures::future::{loop_fn, Future, Loop};
use tokio_timer::Delay;

fn main() {
    actix::run(
        || {
            let conn = client::ClientConnector::default().start();
            loop_fn((), move |_| {
                client::get("http://www.rust-lang.org")   // <- Create request builder
                    .header("User-Agent", "Actix-web")
                    .with_connector(conn.clone())
                    .finish().unwrap()
                    .send()                               // <- Send http request
                    .map_err(|_| ())
                    .and_then(|response| {                // <- server http response
                        println!("Response: {:?}", response);
                        Delay::new(Instant::now() + Duration::from_millis(5000)).map_err(|_| ())
                    })
                    .and_then(|_| Ok(Loop::Continue(())))
            })
        }
    );
}
