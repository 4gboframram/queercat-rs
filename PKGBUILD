pkgname="queercat"
pkgver="0.1.3"
pkgrel=1
pkgdesc="Pride flags in the terminal"
arch=("x86_64")
url="https://github.com/4gboframram/queercat-rs/"
license=("Unlicense")
makedepends=("cargo")

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}
build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --workspace --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/queercat"
}
