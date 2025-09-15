use clap::{arg, command, Command, value_parser, ArgAction};
mod commands;

fn build_cli() -> Command {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .arg_required_else_help(true)
                .about("Run a script")
                .arg(
                    arg!(<SCRIPT>)
                        .help("Define the folder or script to be ran")
                        .required(true),
                )
                .arg(
                    arg!([SUBSCRIPT])
                        .help("Define the script to be ran inside a folder")
                        .required(false),
                )
                .arg(
                    arg!([EXTRAS] ...)
                        .help("Arguments passed to the script")
                        .trailing_var_arg(true),
                ),
        )
        .subcommand(
            Command::new("recommend")
                .about("List and check recommended software")
        )
        .subcommand(
            Command::new("clone")
                .arg_required_else_help(true)
                .about("Clone git repos with submodules")
                .arg(
                    arg!([REPOSITORY])
                        .help("Repository to clone")
                        .required(true),
                )
                .arg(
                    arg!(-j --threads)
                        .help("Number of threads")
                        .value_parser(value_parser!(i32))
                        .default_value("1")
                        .action(ArgAction::Set)
                )
                .arg(
                    arg!([EXTRAS] ...)
                        .help("Arguments passed to git")
                        .trailing_var_arg(true)
                ),
        )
}

fn main() {
    let matches = build_cli().get_matches();
    
    match matches.subcommand() {
        Some(("run", sub_matches)) => commands::run::run(
            sub_matches.get_one::<String>("SCRIPT"),
            sub_matches.get_one::<String>("SUBSCRIPT"),
            sub_matches.get_many::<String>("EXTRAS")
        ),
        Some(("recommend", _)) => commands::recommend::recommend(),
        Some(("clone", sub_matches)) => commands::clone::clone(
            sub_matches.get_one::<String>("REPOSITORY"),
            sub_matches.get_one::<i32>("threads"),
            sub_matches.get_many::<String>("EXTRAS")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
