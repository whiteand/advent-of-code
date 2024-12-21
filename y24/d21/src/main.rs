use std::io::Read;

use tracing_subscriber::layer::SubscriberExt;
use y24d21::solve;

fn main() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::FmtSubscriber::builder()
            .without_time()
            .finish()
            .with(tracing_subscriber::EnvFilter::from_default_env()),
    );

    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut instant = std::time::Instant::now();

    let result = solve::<2>(&input);
    println!("Part 1: {}", result);
    println!("Time: {:?}", instant.elapsed());

    println!();

    instant = std::time::Instant::now();
    let result = solve::<25>(&input);
    println!("Part 2: {}", result);
    println!("Time: {:?}", instant.elapsed());
}
