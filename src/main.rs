mod cli;
mod error;
mod git;
mod plugin;
mod subcommand;

pub use error::{Error, Result};

use once_cell::sync::OnceCell;
use std::env;
use std::path::PathBuf;

static BASE_DIR: OnceCell<PathBuf> = OnceCell::new();
static NPK_DIR: OnceCell<PathBuf> = OnceCell::new();
static NPK_CONFIG_DIR: OnceCell<PathBuf> = OnceCell::new();
static NPK_FILE: OnceCell<PathBuf> = OnceCell::new();
static NPK_FTPLUGIN_DIR: OnceCell<PathBuf> = OnceCell::new();

fn main() {
    BASE_DIR.set(
        env::var("NVIM_RUNTIME_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = dirs::home_dir().expect("No home directory found");
                home.join(".local").join("share").join("nvim").join("site")
            }),
    );
    NPK_DIR.set(BASE_DIR.get().unwrap().join("pack"));
    NPK_CONFIG_DIR.set(BASE_DIR.get().unwrap().join(".npk"));
    NPK_FILE.set(BASE_DIR.get().unwrap().join("npk.ron"));
    NPK_FTPLUGIN_DIR.set(BASE_DIR.get().unwrap().join("ftplugin"));

    match cli::build_cli().get_matches().subcommand_name() {
        Some("list") => todo!(),
        Some("install") => todo!(),
        //("uninstall", Some(m)) => cmd::uninstall::exec(m),
        //("config", Some(m)) => cmd::config::exec(m),
        //("move", Some(m)) => cmd::move_cmd::exec(m),
        //("update", Some(m)) => cmd::update::exec(m),
        //("generate", Some(m)) => cmd::generate::exec(m),
        //("completions", Some(m)) => {
        //    let shell = m.value_of("SHELL").unwrap();
        //    cli::build_cli().gen_completions_to("pack", shell.parse().unwrap(), &mut io::stdout());
        //}
        _ => todo!(),
    }
}
