//! Metrics are printed at the end of every cycle as scope is dropped

use std::thread::sleep;
use std::time::Duration;

use dipstick::*;

fn main() {
    let input = Stream::write_to_stdout().buffered(Buffering::Unlimited);

    loop {
        println!("\n------- open scope");

        let metrics = input.metrics();

        metrics.marker("marker_a").mark();

        sleep(Duration::from_millis(1000));

        println!("------- close scope: ");
    }
}
