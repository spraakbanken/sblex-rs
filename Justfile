quick-dev:
    cargo watch -q -c -w server/examples/quick_dev.rs -x 'run --example quick_dev'

serve-dev:
    cargo watch -q -c -w server/src/ -x 'run -p sblex-server'
