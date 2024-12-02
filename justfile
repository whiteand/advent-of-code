
default:
    just --list

generate year day:
    cargo generate --init --path ./template -d year={{year}} -d day={{day}}
    nu fetch.nu 20{{year}} {{day}};

clippy:
    cargo watch -x clippy

clear:
    rm -rf ./**/target
