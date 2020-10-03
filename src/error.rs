use std::fmt;
use std::io;
use std::path::Path;
use std::path::StripPrefixError;
use std::result::Result as StdResult;

//use ron::error::Error;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Format,
    Git(String),
    Editor,
    Build(String),
    PluginNotInstalled,
    NoPlugin,
    InvalidPluginName,
    SkipLocal,
    PluginInstalled(String),
    PackFile(String),
    CopyDir(String),
    Ron(String),
}

impl Error {
    pub fn copy_dir(s: &str) -> Error {
        Error::CopyDir(format!("Fail to copy directory: {}", s))
    }

    pub fn build<T: AsRef<str>>(s: T) -> Error {
        Error::Build(format!("Fail to build plugin: {}", s.as_ref()))
    }

    pub fn plugin_installed<T: AsRef<Path>>(s: T) -> Error {
        Error::PluginInstalled(format!("Plugin already installed under {:?}", s.as_ref()))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::Git(format!("{}", err))
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Error {
        Error::copy_dir(&format!("{}", err))
    }
}

impl From<StripPrefixError> for Error {
    fn from(err: StripPrefixError) -> Error {
        Error::copy_dir(&format!("{}", err))
    }
}

/*
impl From<RonDeError> for Error {
    fn from(_: RonDeError) -> Error {
        Error::LoadRon
    }
}
*/

impl From<ron::error::Error> for Error {
    fn from(err: ron::Error) -> Error {
        Error::Ron(format!("{}", err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Format => write!(f, "Invalid format"),
            Error::Editor => write!(f, "Can not open editor"),
            Error::PluginNotInstalled => write!(f, "Plugin not installed"),
            Error::NoPlugin => write!(f, "Can not find such plugin"),
            Error::InvalidPluginName => write!(f, "Invalid plugin name"),
            Error::SkipLocal => write!(f, "Local plugin. Skipping"),
            Error::Io(ref e) => write!(f, "{}", e),
            Error::Build(ref s)
            | Error::Git(ref s)
            | Error::CopyDir(ref s)
            | Error::PluginInstalled(ref s)
            | Error::Ron(ref s)
            | Error::PackFile(ref s) => write!(f, "{}", s),
        }
    }
}
