use std::path::PathBuf;

use cfg::{self, Environment};
use failure::Error;
use serde::Deserialize;

pub trait Config<'d>: Sized + Deserialize<'d> {
    fn new(file: Option<PathBuf>) -> Result<Self, Error> {
        let mut c = cfg::Config::new();

        // optionally load from file if provided
        match file {
            Some(path) => {
                c.merge::<cfg::File<_>>(path.into())?;
            }
            None => {
                // hardcode in the filename 'librectf.toml'
                let path = PathBuf::from("librectf.toml");
                if path.exists() {
                    c.merge::<cfg::File<_>>(path.into())?;
                }
            }
        }

        c.merge(Environment::with_prefix("librectf"))?;
        c.try_into().map_err(|err| err.into())
    }
}

impl<'d, T: Deserialize<'d>> Config<'d> for T {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisConfig {
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreConfig {
    redis: RedisConfig,
}
