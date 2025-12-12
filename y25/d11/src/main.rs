use std::io::Read;
use tracing_subscriber::layer::SubscriberExt;
use y25d11::{part1, part2};

fn main() {
    // let (chrome_layer, _guard) = tracing_chrome::ChromeLayerBuilder::new().build();
    // tracing_subscriber::util::SubscriberInitExt::init(
    //     tracing_subscriber::layer::SubscriberExt::with(
    //         tracing_subscriber::registry(),
    //         chrome_layer,
    //     ),
    // );
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

    let result = part1(&input);
    println!("Part 1: {}", result);
    println!("Time: {:?}", instant.elapsed());

    println!();

    instant = std::time::Instant::now();
    let result = part2(&input);
    println!("Part 2: {}", result);
    println!("Time: {:?}", instant.elapsed());
}
