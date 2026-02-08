//! Podlet generates [Podman](https://podman.io/)
//! [Quadlet](https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html)
//! (systemd-like) files from a Podman command, compose file, or existing object.
//!
//! # Usage
//!
//! ```shell
//! $ podlet podman run quay.io/podman/hello
//! [Container]
//! Image=quay.io/podman/hello
//! ```
//!
//! Run `podlet --help` for more information.

mod cli;
mod escape;
mod quadlet;
mod serde;
#[macro_use] extern crate rocket;

const INDEX_HTML: &str = include_str!("index.html");

use clap::Parser;
use color_eyre::eyre;
use rocket::response::content::RawHtml;
use self::cli::Cli;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    return RawHtml(INDEX_HTML)
}

#[launch]
fn webmain() -> _ {
    rocket::build().mount("/", routes![index, hello])
}

/*fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    Cli::parse().print_or_write_files()
}*/
