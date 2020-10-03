use super::Plugin;

use crate::{Error, Result, NPK_DIR};

use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;

/*
pub struct Plugin {
    pub name: String,
    /// Specifies commands which load this plugin.
    pub command: Option<String>,
    /// Specifies filetypes which load this plugin.
    pub filetype: Option<String>,
    /// Specifies maps which load this plugin. See "Keybindings".
    pub keys: Option<String>,
    /// Specifies autocommand events which load this plugin
    pub event: Option<String>,
    pub build: Option<String>,
    /// Specifies a what category to place plugin in
    /// Defaults to plugin author
    pub category: String,
    /// Specifies amount of threads to use
    pub threads: usize,
    pub local: bool,
}
*/

impl Plugin {
    pub fn install() {}

    pub fn repo_path(&self) -> PathBuf {
        match &self.opt {
            true => NPK_DIR
                .get()
                .expect("Error: unable to get PathBuf NPK_DIR")
                .join(&self.category)
                .join("opt")
                .join(&self.name),
            false => NPK_DIR
                .get()
                .expect("Error: unable to get PathBuf NPK_DIR")
                .join(&self.category)
                .join("start")
                .join(&self.name),
        }
    }

    pub fn is_installed(&self) -> bool {
        self.repo_path().is_dir()
    }

    pub fn try_build(&self) -> Result<()> {
        if let Some(ref c) = &self.build {
            let path = self.repo_path();
            let p = process::Command::new("sh")
                .arg("-c")
                .arg(c)
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .current_dir(&path)
                .spawn()?;
            let output = p.wait_with_output()?;
            if !output.status.success() {
                let err = String::from_utf8(output.stderr)
                    .unwrap_or_else(|_| String::from("No error output!"));
                return Err(Error::Build(err));
            }
        }
        Ok(())
    }

    pub fn for_filetypes(&self) {
        if let Some(ft) = &self.filetype {
            if !ft.is_empty() {
                let filetypes = ft.split(',').collect::<Vec<&str>>();
                for filetype in filetypes {
                    todo!()
                }
            }
        }
    }
}
