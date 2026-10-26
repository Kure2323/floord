pub mod service;
pub mod floord_args;
pub mod database;
pub mod file_groups;

fn main() {
    let arguments = floord_args::parse_args();
    service::run(arguments);
}