const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = 8;
    let ready = 2;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    print!("{} missiles left ", missiles);

    println!(
        "Firing {} of my {} missiles...",
        STARTING_MISSLES, READY_AMOUNT
    );
}
