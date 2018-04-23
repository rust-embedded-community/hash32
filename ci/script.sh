set -euxo pipefail

main() {
    cargo check

    case $TARGET in
        x86_64-unknown-linux-gnu)
            cargo test
            ;;
    esac

    [ $TRAVIS_RUST_VERSION = nightly ] && cargo check --features const-fn
}

main
