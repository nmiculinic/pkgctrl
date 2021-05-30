use handlebars::Handlebars;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

static PKGBUILD_TEMPLATE: &'static str = r#"
# Maintainer: Neven Miculinic <neven.miculinic@gmail.com>

depends=('glibc' 'pacman' 'yay')
pkgname=pkgctrl-bin
pkgdesc="reconcile packages installed on Arch Linux system"
pkgver={{pkgver}}
pkgrel=1
arch=('x86_64')
url="https://github.com/nmiculinic/pkgctrl"
license=('Apache')
provides=('pkgctrl')
conflicts=('pkgctrl')
_binary=pkgctrl-linux-amd64
source=("$_binary-$pkgver::https://github.com/nmiculinic/pkgctrl/releases/download/v$pkgver/$_binary")
sha256sums=('{{sha256sum}}')

package() {
  install -Dm 755 "$srcdir/$_binary-$pkgver" "$pkgdir/usr/bin/pkgctrl"
}
"#;

#[derive(Serialize)]
struct Values {
    pkgver: String,
    sha256sum: String,
}

#[derive(StructOpt, Serialize)]
#[structopt(name = "ci", about = "generate PKGBUILD")]
struct CLI {
    #[structopt(long, parse(from_os_str))]
    pkgctrl_path: PathBuf,

    #[structopt(long, parse(from_os_str))]
    pkgbuild_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = CLI::from_args();
    let input = std::fs::read(opt.pkgctrl_path)?;
    let digest = Sha256::digest(&input);

    let values = Values {
        pkgver: env!("CARGO_PKG_VERSION").to_string(),
        sha256sum: format!("{:x}", digest),
    };

    println!("sha256sum: {:x}", digest);

    let reg = Handlebars::new();
    let output = File::create(opt.pkgbuild_path)?;
    write!(
        &output,
        "{}",
        reg.render_template(PKGBUILD_TEMPLATE, &values,)?
    )?;
    Ok(())
}
