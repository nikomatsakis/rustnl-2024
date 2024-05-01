use std::sync::Arc;

use clap::Parser;
use eg_lang::FormalityLang;
use fn_error_context::context;
use formality_core::Fallible;
use grammar::Program;

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
            "u32",
            "let",
        ];
    }
}

#[derive(Parser, Debug)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Args {
    paths: Vec<String>,
}

pub fn main() -> Fallible<()> {
    let args = Args::try_parse()?;

    for path in &args.paths {
        check_file(path)?;
    }

    Ok(())
}

#[context("check input file `{path:?}`")]
fn check_file(path: &str) -> Fallible<()> {
    let text: String = std::fs::read_to_string(path)?;
    check_program(&text)
}

#[context("check program")]
fn check_program(text: &str) -> Fallible<()> {
    // HACK: disable backtraces from anyhow.
    std::env::set_var("RUST_LIB_BACKTRACE", "0");

    let program: Arc<Program> = eg_lang::try_term(text)?;
    type_system::check_program(&program)?;
    Ok(())
}
