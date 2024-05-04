use std::sync::Arc;

use clap::Parser;
use eg_lang::FormalityLang;
use execute::Value;
use fn_error_context::context;
use formality_core::Fallible;
use grammar::Program;

mod execute;
mod grammar;
mod type_system;

#[cfg(test)]
mod test;

formality_core::declare_language! {
    mod eg_lang {
        const NAME = "Eg";
        type Kind = crate::grammar::Kind;
        type Parameter = crate::grammar::Parameter;
        const BINDING_OPEN = '<';
        const BINDING_CLOSE = '>';
        const KEYWORDS = [
            "fn",
            "type",
            "u32",
            "let",
        ];
    }
}

#[derive(Parser, Debug)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    execute: Option<String>,
    paths: Vec<String>,
}

pub fn main() -> Fallible<()> {
    let args = Args::try_parse()?;

    if let Some(execute) = &args.execute {
        let value = execute_program(execute)?;
        println!("{value:#?}");
    }

    for path in &args.paths {
        let value = execute_file(path)?;
        println!("{value:#?}");
    }

    Ok(())
}

#[context("check input file `{path:?}`")]
fn execute_file(path: &str) -> Fallible<Value> {
    let text: String = std::fs::read_to_string(path)?;
    execute_program(&text)
}

#[context("check program")]
fn execute_program(text: &str) -> Fallible<Value> {
    // HACK: disable backtraces from anyhow.
    std::env::set_var("RUST_LIB_BACKTRACE", "0");

    let program: Arc<Program> = eg_lang::try_term(text)?;
    type_system::check_program(&program)?;
    execute::execute(&program)
}
