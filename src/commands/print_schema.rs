use clap::{Args, Subcommand};
use schemars::schema_for;

use crate::{commands::version::PostVersionInfo, common::config::Config};

#[derive(Args, Debug)]
pub(crate) struct PrintSchemaArgs {
    #[command(subcommand)]
    schema: Schema,
}

#[derive(Debug, Subcommand)]
enum Schema {
    Config,
    #[clap(name = "postVersion")]
    PostVersion,
}

pub fn print_schema(args: PrintSchemaArgs) {
    let schema = match args.schema {
        Schema::Config => schema_for!(Config),
        Schema::PostVersion => schema_for!(PostVersionInfo),
    };
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
