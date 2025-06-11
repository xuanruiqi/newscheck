# Maintainer: Xuanrui Qi <me@xuanruiqi.com>
pkgname=newscheck
pkgver=0.1.0-alpha
pkgrel=1
pkgdesc="Yet another Arch Linux news reader"
url="https://github.com/xuanruiqi/newscheck"
arch=('x86_64')
license=('MIT')
makedepends=('cargo')

prepare() {
  cd "$pkgname-$pkgver"
  cargo fetch --locked --target "$(rustc -vV | sed -n 's|host: ||p')"
}

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --frozen --features "${_features:-}" --release --target-dir target
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 target/release/newscheck "${pkgdir}/usr/bin/newscheck"
  # Generate completion files.
  mkdir -p "$pkgdir/usr/share/bash-completion/completions"
  "$pkgdir"/usr/bin/newscheck completions bash > "$pkgdir/usr/share/bash-completion/completions/newscheck"
  mkdir -p "$pkgdir/usr/share/fish/vendor_completions.d"
  "$pkgdir"/usr/bin/newscheck completions fish > "$pkgdir/usr/share/fish/vendor_completions.d/newscheck.fish"
  mkdir -p "$pkgdir/usr/share/zsh/site-functions"
  "$pkgdir"/usr/bin/newscheck completions zsh > "$pkgdir/usr/share/zsh/site-functions/_newscheck"
}