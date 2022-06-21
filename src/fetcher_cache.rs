use std::path::PathBuf;

pub fn subcommand<'a, 'b>() -> clap::App<'a, 'b> {
    clap::SubCommand::with_name("fetcher-cache")
        .about("Query and manipulate the Nix fetcher cache")
        .subcommand(clap::SubCommand::with_name("clear").about("Clear the Nix fetcher cache"))
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
}

pub fn handle(matches: &clap::ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("clear", _) => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("nix").unwrap();
            let cache_path: PathBuf = xdg_dirs.get_cache_home();
            let res = cache_path
                .read_dir()
                .expect("Failed to read cache directory")
                .flatten()
                .flat_map(|ent| {
                    if ent
                        .file_name()
                        .into_string()
                        .unwrap()
                        .contains("fetcher-cache")
                    {
                        Some(ent.path())
                    } else {
                        None
                    }
                })
                .map(|cache_file| {
                    println!("Clearing {:?}", cache_file);
                    std::fs::remove_file(cache_file)
                })
                .collect::<Result<(), _>>()
                .map_err(|e| e.to_string());
            if res.is_ok() {
                println!("fetcher cache cleared")
            }
            res
        }
        (_, _) => unreachable!(),
    }
}
