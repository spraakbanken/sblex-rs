quick-dev:
    cargo watch -q -c -w server/examples/quick_dev.rs -x 'run --example quick_dev'

serve-dev:
    cargo watch -q -c -w server/src/ -x 'run -p sblex-server'

test:
    cargo test

watch-trie-map-test:
    cargo watch -q -c -w trie-map -x 'test -p trie-map'

serve-fm-server:
    cargo watch -q -c -w fm-server/src/ -x 'run -p fm-server'

quick-dev-fm-server:
    cargo watch -q -c -w fm-server/examples/ -x 'run -p fm-server --example fm_server_quick_dev'