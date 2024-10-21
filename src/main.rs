use std::io;

use std::process::ExitCode;

fn sub() -> Result<(), io::Error> {
    rs_ltsv2labels::label::stdin2stats2stdout_default()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}