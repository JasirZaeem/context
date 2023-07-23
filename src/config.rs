use std::path::{PathBuf};

use crate::arguments::ContextArgs;
use anyhow::{Result, anyhow, Context};

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
    Config,
}

#[derive(Debug)]
pub struct ContextConfig {
    pub pwd: PathBuf,
    pub config: PathBuf,
    pub operation: Operation,
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(mut value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Ok(Self::Print(None));
        }

        let mut args = value.drain(..);

        match args.next().as_deref() {
            Some("add") => {
                if args.len() != 2 {
                    return Err(anyhow!("Wrong number of arguments for add"));
                }
                Ok(Self::Add(
                    args.next().expect("Missing key to add"),
                    args.next().expect("Missing value to add")))
            }
            Some("rm") => {
                if args.len() != 1 {
                    return Err(anyhow!("Wrong number of arguments for remove"));
                }
                Ok(Self::Remove(args.next().expect("Missing key to remove")))
            }
            Some("print") => {
                Ok(Self::Print(args.next()))
            }
            Some("config") => {
                if args.len() != 0 {
                    return Err(anyhow!("Wrong number of arguments for config"));
                }

                Ok(Self::Config)
            }
            Some(key) => {
                if args.len() != 0 {
                    return Err(anyhow!("Wrong number of arguments for {}", key));
                }

                Ok(Self::Print(Some(key.to_string())))
            }
            None => Err(anyhow!("Missing operation"))
        }
    }
}

fn get_config_path_or_default(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(config) = config {
        return Ok(config);
    }

    if let Ok(home) = std::env::var("CONTEXT_CONFIG") {
        return Ok(PathBuf::from(home));
    }

    if let Ok(home) = std::env::var("XDG_CONFIG_HOME") {
        let mut path = PathBuf::from(home);
        path.push("context");
        path.push("context_config.json");
        return Ok(path);
    }

    if let Ok(home) = std::env::var("HOME") {
        let mut path = PathBuf::from(home);
        path.push("context");
        path.push("context_config.json");
        return Ok(path);
    }

    Err(anyhow!("Unable to find config file"))
}

fn get_pwd_or_default(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    std::env::current_dir().context("Unable to get current directory")
}


impl TryFrom<ContextArgs> for ContextConfig {
    type Error = anyhow::Error;

    fn try_from(args: ContextArgs) -> Result<Self, Self::Error> {
        Ok(ContextConfig {
            operation: args.operation.try_into()?,
            config: get_config_path_or_default(args.config)?,
            pwd: get_pwd_or_default(args.pwd)?,
        })
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::config::{Operation};
    use crate::arguments::ContextArgs;
    use super::ContextConfig;

    #[test]
    fn test_implicit_print_all() -> Result<()> {
        let config: ContextConfig = ContextArgs {
            pwd: None,
            config: None,
            operation: vec![],
        }.try_into()?;

        assert_eq!(config.operation, Operation::Print(None));
        Ok(())
    }

    #[test]
    fn test_print_all() -> Result<()> {
        let config: ContextConfig = ContextArgs {
            pwd: None,
            config: None,
            operation: vec!["print".to_string()],
        }.try_into()?;

        assert_eq!(config.operation, Operation::Print(None));
        Ok(())
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let config: ContextConfig = ContextArgs {
            pwd: None,
            config: None,
            operation: vec!["print".to_string(), "key".to_string()],
        }.try_into()?;

        assert_eq!(config.operation, Operation::Print(Some("key".to_string())));
        Ok(())
    }

    #[test]
    fn test_add_key_value() -> Result<()> {
        let config: ContextConfig = ContextArgs {
            pwd: None,
            config: None,
            operation: vec!["add".to_string(), "key".to_string(), "value".to_string()],
        }.try_into()?;

        assert_eq!(config.operation, Operation::Add("key".to_string(), "value".to_string()));
        Ok(())
    }

    #[test]
    fn test_remove_key() -> Result<()> {
        let config: ContextConfig = ContextArgs {
            pwd: None,
            config: None,
            operation: vec!["rm".to_string(), "key".to_string()],
        }.try_into()?;

        assert_eq!(config.operation, Operation::Remove("key".to_string()));
        Ok(())
    }
}