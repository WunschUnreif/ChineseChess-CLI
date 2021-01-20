#[macro_use]
extern crate lazy_static;

mod model;
mod dispatcher;
mod config;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    dispatcher::run_server()?;

    Ok(())
}
