use clap::Parser;
use context::arguments::ContextArgs;
use context::config::{ContextConfig, Operation};
use anyhow::Result;
use context::context::Context;

fn main() -> Result<()> {
    let config: ContextConfig = ContextArgs::parse().try_into()?;
    let mut context = Context::from_config_props(config.config, config.pwd);

    match config.operation {
        Operation::Print(None) => {
            serde_json::to_writer_pretty(std::io::stdout(), &context.get_value_all())?;
        }
        Operation::Print(Some(k)) => {
            serde_json::to_writer_pretty(std::io::stdout(), &context.get_value(&k))?;
        }
        Operation::Add(k, v) => {
            context.set_value(k, v);
            context.save()?;
        }
        Operation::Remove(k) => {
            context.remove_value(&k);
            context.save()?;
        }
        Operation::Config => {
            println!("{:?}", context.config_path());
        }
    };

    Ok(())
}
