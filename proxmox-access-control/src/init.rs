use anyhow::{format_err, Error};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::OnceLock,
};

static ACCESS_CONF: OnceLock<&'static dyn AccessControlConfig> = OnceLock::new();
static ACCESS_CONF_DIR: OnceLock<PathBuf> = OnceLock::new();

/// This trait specifies the functions a product needs to implement to get ACL tree based access
/// control management from this plugin.
pub trait AccessControlConfig: Send + Sync {
    /// Returns a mapping of all recognized privileges and their corresponding `u64` value.
    fn privileges(&self) -> &HashMap<&str, u64>;

    /// Returns a mapping of all recognized roles and their corresponding `u64` value.
    fn roles(&self) -> &HashMap<&str, u64>;

    /// Optionally returns a role that has no access to any resource.
    ///
    /// Default: Returns `None`.
    fn role_no_access(&self) -> Option<&str> {
        None
    }

    /// Optionally returns a role that is allowed to access all resources.
    ///
    /// Default: Returns `None`.
    fn role_admin(&self) -> Option<&str> {
        None
    }
}

pub fn init<P: AsRef<Path>>(
    acm_config: &'static dyn AccessControlConfig,
    config_dir: P,
) -> Result<(), Error> {
    init_access_config(acm_config)?;
    init_access_config_dir(config_dir)
}

pub(crate) fn init_access_config_dir<P: AsRef<Path>>(config_dir: P) -> Result<(), Error> {
    ACCESS_CONF_DIR
        .set(config_dir.as_ref().to_owned())
        .map_err(|_e| format_err!("cannot initialize acl tree config twice!"))
}

pub(crate) fn init_access_config(config: &'static dyn AccessControlConfig) -> Result<(), Error> {
    ACCESS_CONF
        .set(config)
        .map_err(|_e| format_err!("cannot initialize acl tree config twice!"))
}

pub(crate) fn access_conf() -> &'static dyn AccessControlConfig {
    *ACCESS_CONF
        .get()
        .expect("please initialize the acm config before using it!")
}

fn conf_dir() -> &'static PathBuf {
    ACCESS_CONF_DIR
        .get()
        .expect("please initialize acm config dir before using it!")
}

pub(crate) fn acl_config() -> PathBuf {
    conf_dir().join("acl.cfg")
}

pub(crate) fn acl_config_lock() -> PathBuf {
    conf_dir().join(".acl.lck")
}

