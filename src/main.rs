use clap::{App, AppSettings};

mod fetcher_cache;
mod flake;
mod gcroots;
mod gen;

fn main() {
    let matches = App::new("nix-utils")
        .about("Various extra Nix utilities")
        .subcommand(gen::subcommand())
        .subcommand(gcroots::subcommand())
        .subcommand(flake::subcommand())
        .subcommand(fetcher_cache::subcommand())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    let res = match matches.subcommand() {
        ("gen", Some(matches)) => gen::handle(matches),
        ("gcroots", Some(matches)) => gcroots::handle(matches),
        ("flake", Some(matches)) => flake::handle(matches),
        ("fetcher-cache", Some(matches)) => fetcher_cache::handle(matches),
        (_, _) => unreachable!(),
    };

    if let Err(msg) = res {
        eprintln!("{}", msg);
        std::process::exit(1);
    }
}
