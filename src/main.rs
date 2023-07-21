use clap::Parser;
use context::arguments::ContextArgs;
use context::config::ContextConfig;


fn main() {
    let args = ContextArgs::parse();
    println!("{:?}", args);

    let config: ContextConfig = args.try_into().unwrap();
    println!("{:?}", config);
}
