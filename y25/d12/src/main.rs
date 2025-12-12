use std::io::Read;
use tracing_subscriber::layer::SubscriberExt;
use y25d12::part1;

fn main() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::FmtSubscriber::builder()
            .finish()
            .with(tracing_subscriber::EnvFilter::from_default_env()),
    );

    // read stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let instant = std::time::Instant::now();

    let result = part1(&input);
    println!("Part 1: {}", result);
    println!("Time: {:?}", instant.elapsed());
}
