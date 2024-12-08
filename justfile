
default:
    just --list

generate year day:
    /Users/whiteand/.cargo/bin/cargo generate --init --path ./template -d year={{year}} -d day={{day}} --name y{{year}}d{{day}}
    /opt/homebrew/bin/nu fetch.nu 20{{year}} {{day}};
    /opt/homebrew/bin/deno --allow-read --allow-write ./scripts/add-member {{year}} {{day}};

clippy:
    cargo watch -x clippy

clear:
    rm -rf ./**/target
