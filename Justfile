quick-dev:
    cargo watch -q -c -w server/examples/quick_dev.rs -x 'run --example quick_dev'

serve-dev:
    cargo watch -q -c -w server/src/ -x 'run -p sblex-server'

test:
    cargo test

watch-trie-map-test:
    cargo watch -q -c -w trie-map -x 'test -p trie-map'

serve-fm-server:
    cargo watch -q -c -w crates/fm-server/src -x 'run -p fm-server'

quick-dev-fm-server:
    cargo watch -q -c -w crates/fm-server/examples/ -x 'run -p fm-server --example fm_server_quick_dev'

build-load-morphology:
    cargo build --profile profiling --example load_morphology

mprof-load-morphology INPUT='': build-load-morphology
    mprof run target/profiling/examples/load_morphology {{INPUT}}