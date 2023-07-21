use clap::Parser;
use context::arguments::ContextArgs;


fn main() {
    let args = ContextArgs::parse();

    println!("{:?}", args);
}
