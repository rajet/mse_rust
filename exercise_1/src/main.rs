mod advprpa;

use advprpa::compute_message;

fn main() {
    let msg = compute_message();
    println!("{}", msg);
}
