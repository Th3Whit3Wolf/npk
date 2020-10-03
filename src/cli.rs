use clap::{App, Arg};

pub fn build_cli() -> clap::App<'static> {
    App::new("pack")
        .about("Package manager for vim")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .subcommand(
            App::new("list")
                .about("List installed packages")
                .arg(
                    Arg::new("start")
                        .long("start")
                        .short('s')
                        .conflicts_with("opt")
                        .about("List start packages"),
                )
                .arg(
                    Arg::new("opt")
                        .long("opt")
                        .short('c')
                        .conflicts_with("start")
                        .about("List optional packages"),
                )
                .arg(
                    Arg::new("detached")
                        .long("detached")
                        .short('c')
                        .about("List detached(untracked) packages"),
                )
                .arg(
                    Arg::new("category")
                        .long("category")
                        .short('c')
                        .about("List packages under this category")
                        .value_name("CATEGORY"),
                ),
        )
        .subcommand(
            App::new("install")
                .about("Install new packages/plugins")
                .arg(
                    Arg::new("opt")
                        .short('o')
                        .long("opt")
                        .about("Install plugins as opt(ional)"),
                )
                .arg(
                    Arg::new("category")
                        .long("category")
                        .short('c')
                        .about("Install package under provided category")
                        .default_value("default")
                        .value_name("CATEGORY"),
                )
                .arg(
                    Arg::new("local")
                        .short('l')
                        .long("local")
                        .about("Install local plugins"),
                )
                .arg(
                    Arg::new("on")
                        .long("on")
                        .about("Command for loading the plugins")
                        .value_name("LOAD_CMD"),
                )
                .arg(
                    Arg::new("for")
                        .long("for")
                        .about("Load this plugins for specific types")
                        .value_name("TYPES"),
                )
                .arg(
                    Arg::new("build")
                        .long("build")
                        .about("Build command for build package")
                        .value_name("BUILD_CMD"),
                )
                .arg(
                    Arg::new("threads")
                        .short('j')
                        .long("threads")
                        .about("Installing packages concurrently")
                        .value_name("THREADS"),
                )
                .arg(Arg::new("package").multiple(true)),
        )
    /*
    .subcommand(
        SubCommand::with_name("uninstall")
            .about("Uninstall packages/plugins")
            .arg(
                Arg::with_name("all")
                    .short("a")
                    .long("all")
                    .help("remove all package related configuration as well"),
            )
            .arg(Arg::with_name("package").required(true).multiple(true)),
    )
    .subcommand(
        SubCommand::with_name("config")
            .about("Configure/edit the package specific configuration")
            .arg(
                Arg::with_name("delete")
                    .short("d")
                    .long("delete")
                    .help("Delete package configuration file"),
            )
            .arg(Arg::with_name("package").required(true)),
    )
    .subcommand(
        SubCommand::with_name("move")
            .about("Move a package to a different category or make it optional.")
            .arg(
                Arg::with_name("opt")
                    .conflicts_with("category")
                    .long("opt")
                    .short("o")
                    .help("Make package optional"),
            )
            .arg(
                Arg::with_name("package")
                    .help("Package to move")
                    .required(true),
            )
            .arg(
                Arg::with_name("category")
                    .conflicts_with("opt")
                    .help("Category to move the package to"),
            ),
    )
    .subcommand(
        SubCommand::with_name("update")
            .about("Update packages")
            .arg(
                Arg::with_name("skip")
                    .short("s")
                    .long("skip")
                    .multiple(true)
                    .help("Skip packages"),
            )
            .arg(
                Arg::with_name("packfile")
                    .short("p")
                    .long("packfile")
                    .help("Regenerate the '_pack' file (combine all package configurations)"),
            )
            .arg(
                Arg::with_name("threads")
                    .short("j")
                    .long("threads")
                    .help("Updating packages concurrently"),
            )
            .arg(
                Arg::with_name("package")
                    .help("Packages to update, default all")
                    .multiple(true),
            ),
    )
    .subcommand(
        SubCommand::with_name("generate")
            .about("Generate the pack package file")
            .help("Generate _pack.vim file which combines all package configurations"),
    )
    .subcommand(
        SubCommand::with_name("completions")
            .about("Generates completion scripts for your shell")
            .setting(AppSettings::Hidden)
            .arg(
                Arg::with_name("SHELL")
                    .required(true)
                    .possible_values(&["bash", "fish", "zsh"])
                    .help("The shell to generate the script for"),
            ),
    )
    */
}
