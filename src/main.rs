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

use color_eyre::eyre::{self, ensure};
use compose_spec::{Compose, Resource};
use rocket::response::content::RawHtml;
use self::cli::compose::parts_try_into_files;
use self::quadlet::JoinOption;

const INDEX_HTML: &str = include_str!("index.html");

pub fn convert_compose_to_quadlet(compose_yaml: &str) -> eyre::Result<String> {
    // 1. Parse YAML
    let mut options = Compose::options();
    options.apply_merge(true);
    let compose = options.from_yaml_reader(compose_yaml.as_bytes())?;
    compose.validate_all()?;

    // 2. Deconstruct
    let Compose {
        version: _,
        name: _,
        include,
        services,
        networks,
        volumes,
        configs,
        secrets,
        extensions,
    } = compose;

    // 3. validate
    ensure!(include.is_empty(), "`include` is not supported");
    ensure!(configs.is_empty(), "`configs` is not supported");
    ensure!(
        secrets.values().all(Resource::is_external),
        "only external `secrets` are supported"
    );
    ensure!(extensions.is_empty(), "compose extensions are not supported");

    // 4. convert
    let files = parts_try_into_files(services, networks, volumes, None, None, None)?;

    // 5. compile
    let join_options = JoinOption::all_set();
    let mut out: String = String::new();
    for file in files {
        out += "# ";
        out += file.name();
        out += ".";
        out += file.extension();
        out += "\n";
        out += file.serialize(&join_options)?.as_str();
    }

    Ok(out)
}

#[post("/convert", data = "<input>")]
fn convert(input: String) -> String {
    println!("Received /convert request for {input}");
    let quads = convert_compose_to_quadlet(input.as_str());
    match quads {
        Err(e) => format!("An error occured: {e}"),
        Ok(d) => d,
    }
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    return RawHtml(INDEX_HTML)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, convert])
}
