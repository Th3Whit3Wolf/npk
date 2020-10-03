use crate::git;
use crate::plugin::Plugin;
use crate::{Error, Result};
use clap::ArgMatches;
use num_cpus;
//use std::os::unix::fs::symlink;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct InstallArgs {
    /// Name of plugin
    plugin: String,
    local: bool,
    /// Specifies commands which load this plugin.
    command: Option<String>,
    /// Specifies filetypes which load this plugin.
    filetype: Option<String>,
    /// Specifies maps which load this plugin. See "Keybindings".
    keys: Option<String>,
    /// Specifies autocommand events which load this plugin
    event: Option<String>,
    /// Specifies amount of threads to use
    threads: Option<usize>,
    /// Manually marks a plugin as optional.
    opt: bool,
    /// Specifies a what category to place plugin in
    /// Defaults to plugin author
    category: Option<String>,
    /// Specifies code to run to install/update plugin
    build: Option<String>,
    /// Specifies a git branch to use
    branch: Option<String>,
    /// Specifies a git tag to use
    tag: Option<String>,
    /// Specifies a git commit to use
    commit: Option<String>,
}

impl InstallArgs {
    pub fn from_matches(matches: &ArgMatches) -> InstallArgs {
        InstallArgs {
            plugin: matches.value_of("package").unwrap_or("").to_string(),
            local: matches.is_present("local"),
            command: if let Some(command) = matches.value_of("on") {
                Some(command.to_string())
            } else {
                None
            },
            filetype: if let Some(ft) = matches.value_of("for") {
                Some(ft.to_string())
            } else {
                None
            },
            keys: if let Some(keys) = matches.value_of("keys") {
                Some(keys.to_string())
            } else {
                None
            },
            event: if let Some(event) = matches.value_of("event") {
                Some(event.to_string())
            } else {
                None
            },
            threads: if let Some(threads) = matches.value_of("threads") {
                match threads.parse::<usize>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                }
            } else {
                None
            },
            opt: matches.is_present("opt")
                || matches.value_of("on").is_some()
                || matches.value_of("for").is_some()
                || matches.value_of("keys").is_some()
                || matches.value_of("event").is_some(),
            category: if let Some(cat) = matches.value_of("category") {
                Some(cat.to_string())
            } else {
                None
            },
            build: if let Some(build) = matches.value_of("build") {
                Some(build.to_string())
            } else {
                None
            },
            branch: if let Some(branch) = matches.value_of("branch") {
                Some(branch.to_string())
            } else {
                None
            },
            tag: if let Some(tag) = matches.value_of("tag") {
                Some(tag.to_string())
            } else {
                None
            },
            commit: if let Some(commit) = matches.value_of("commit") {
                Some(commit.to_string())
            } else {
                None
            },
        }
    }

    pub fn to_plugin(self) -> Result<Plugin> {
        if let true = self.plugin.contains('/') {
            let plug_short: Vec<&str> = self.plugin.split('/').collect();
            let threads = match self.threads {
                Some(t) => {
                    if t < 2 {
                        1
                    } else {
                        t
                    }
                }
                _ => num_cpus::get(),
            };
            Ok(Plugin {
                name: plug_short[plug_short.len() - 2].to_string(),
                /// Specifies commands which load this plugin.
                command: self.command,
                /// Specifies filetypes which load this plugin.
                filetype: self.filetype,
                /// Specifies maps which load this plugin. See "Keybindings".
                keys: self.keys,
                /// Specifies autocommand events which load this plugin
                event: self.event,
                build: self.build,
                /// Specifies a what category to place plugin in
                /// Defaults to plugin author
                category: if let Some(cat) = self.category {
                    cat
                } else {
                    plug_short[plug_short.len() - 1].to_string()
                },
                /// Specifies amount of threads to use
                threads,
                /// Manually marks a plugin as optional.
                opt: self.opt,
                local: self.local,
            })
        } else {
            Err(Error::InvalidPluginName)
        }
    }
}
