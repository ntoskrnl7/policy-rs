pub trait Base {
    /// Adjust default values for optional attributes
    fn adjust_default_options(&mut self) -> Self;

    /// Returns the default policy object.
    fn default() -> Self;
}

/// 정책 객체 로더
pub trait Loader<Policy> {
    /// Create a policy object from a file.
    fn from_path<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Policy>;

    /// Create a policy object from text (YAML -> JSON -> TOML)
    fn from_str<T: AsRef<str>>(text: T) -> std::io::Result<Policy>;

    /// Create a policy object from YAML text
    fn from_yaml<T: AsRef<str>>(yaml: T) -> Result<Policy, serde_yaml::Error>;

    /// Create a policy object from JSON text
    fn from_json<T: AsRef<str>>(json: T) -> Result<Policy, serde_json::Error>;

    /// Create a policy object from TOML text
    fn from_toml<T: AsRef<str>>(toml: T) -> Result<Policy, toml::de::Error>;
}

pub trait Notify<Policy> {
    fn notify(&mut self);
}

#[cfg(feature = "policy_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate policy_derive;

#[cfg(feature = "policy_derive")]
#[doc(hidden)]
pub use policy_derive::*;
