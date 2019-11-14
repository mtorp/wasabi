use main_error::MainError;
use std::{fs, io};
use structopt::StructOpt;
use wasabi::instrument::add_hooks;
use wasabi::options::*;
use wasm::ast::highlevel::Module;

// TODO use failure crate and failure::Error type for error handling or use custom error trait

fn main() -> Result<(), MainError> {
    let opt = Options::from_args();

    let mut enabled_hooks = if opt.hooks.is_empty() {
        // if --hooks is not given, everything shall be instrumented.
        HookSet::all()
    } else {
        let mut enabled_hooks = HookSet::new();
        for hook in opt.hooks {
            enabled_hooks.insert(hook);
        }
        enabled_hooks
    };
    for hook in opt.no_hooks {
        enabled_hooks.remove(hook);
    }

    let input_file = opt.input_file;
    let output_dir = opt.output_dir;

    let input_filename_no_ext = input_file.file_stem().ok_or(io_err("invalid input file"))?;
    let mut output_file_stem = output_dir.clone();
    output_file_stem.push(input_filename_no_ext);
    let output_file_wasm = output_file_stem.with_extension("wasm");
    let output_file_js = output_file_stem.with_extension("wasabi.js");

    // instrument Wasm and generate JavaScript
    let mut module = Module::from_file(input_file.clone())?;
    let js = add_hooks(&mut module, &enabled_hooks).unwrap();

    // write output files
    fs::create_dir_all(output_dir)?;
    module.to_file(output_file_wasm)?;
    fs::write(output_file_js, js)?;

    Ok(())
}

// TODO remove after proper error handling
fn io_err(str: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, str.to_string())
}
