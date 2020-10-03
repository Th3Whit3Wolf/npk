pub mod install;
#[derive(Debug, Clone)]
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
    /// Manually marks a plugin as optional.
    pub opt: bool,
    pub local: bool,
}
