use clap::{arg, command, Command};

fn main() {
    let _matches = command!()
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
                        .required(true)
                )
                .arg(
                    arg!([SUBSCRIPT])
                        .help("Define the script to be ran inside a folder")
                        .required(false)
                )
                .arg(
                    arg!([EXTRAS] ...)
                        .help("Arguments passed to git")
                        .trailing_var_arg(true)
                )
        
        )
        .subcommand(
            Command::new("recommend")
                .about("List recommended software")
                .arg(
                    arg!(--check)
                        .help("Check if recommendations are installed")
                        .required(false)
                )
        )
        .subcommand(
            Command::new("clone")
                .arg_required_else_help(true)
                .about("Clone git repos with submodules")
                .arg(
                    arg!([REPOSITORY])
                        .help("Repository to clone")
                        .required(true)
                )
                .arg(
                    arg!([EXTRAS] ...)
                        .help("Arguments passed to git")
                        .trailing_var_arg(true)
                )
        )
        .get_matches();
}
