// main.rs
use futures::executor::block_on;
use repository::run;

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
