use y16d11::solve;

fn main() {
    let (chrome_layer, _guard) = tracing_chrome::ChromeLayerBuilder::new().build();
    tracing_subscriber::util::SubscriberInitExt::init(
        tracing_subscriber::layer::SubscriberExt::with(
            tracing_subscriber::registry(),
            chrome_layer,
        ),
    );

    let mut instant = std::time::Instant::now();

    let result = solve(y16d11::ACTUAL);
    println!("Part 1: {}", result);
    println!("Time: {:?}", instant.elapsed());

    println!();

    instant = std::time::Instant::now();
    let result = solve(y16d11::ACTUAL2);
    println!("Part 2: {}", result);
    println!("Time: {:?}", instant.elapsed());
}
