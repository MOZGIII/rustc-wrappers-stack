use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args_os();
    let Some(_own_command) = args.next() else {
        return ExitCode::FAILURE;
    };
    let Some(first) = args.next() else {
        return ExitCode::SUCCESS;
    };

    let stack = match read_stack() {
        Ok(v) => v,
        Err(error) => {
            eprintln!("Unable to load wrappers stack: {error}");
            return ExitCode::FAILURE;
        }
    };

    let mut process = std::process::Command::new(first);
    process.args(stack);
    process.args(args);
    exec(process)
}

fn read_stack() -> std::io::Result<Vec<String>> {
    Ok(vec![])
}

#[cfg(unix)]
fn exec(mut process: std::process::Command) -> ExitCode {
    use std::os::unix::process::CommandExt;
    let error = process.exec();
    eprintln!("Unable to execute command: {error}");
    ExitCode::FAILURE
}
