# Maintainer: Thomas Gingele (https://github.com/B1TC0R3)
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=hash-cmp-bin
pkgver=4.0.1
pkgrel=1
pkgdesc="Validates the hash of a file against a known hash"
url="https://github.com/B1TC0R3/hash-cmp"
license=("GPL-3.0")
arch=("x86_64")
provides=("hash-cmp")
conflicts=("hash-cmp")
source=("https://github.com/B1TC0R3/hash-cmp/releases/download/v$pkgver/hash-cmp-$pkgver-x86_64.tar.gz")
sha256sums=("962aa11b414482b8a0da40d6b8c4793fadc5a62bd69f2fd89b7693034d43202e")

package() {
    install -Dm755 target/release/hash-cmp -t "$pkgdir/usr/bin/hash-cmp"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 hash-cmp.1.gz "$pkgdir/usr/share/man/man1/hash-cmp.1.gz"
}
