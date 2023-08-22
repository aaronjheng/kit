use std::error;
use std::process;

use clap::{crate_name, crate_version, Arg, ArgAction, Command};
use rust_decimal::Decimal;
use time;

fn command() -> Command {
    Command::new(crate_name!())
    .version(crate_version!())
    .arg(
        Arg::new("append")
            .long("append")
            .short('a').action(ArgAction::SetTrue)
            .help("With -o FILE, append instead of overwriting"),
    )
    .arg(
        Arg::new("format")
            .long("format")
            .short('f')
            .value_name("FORMAT")
            .help("Use the specified FORMAT instead of the default"),
    )
    .arg(
        Arg::new("output")
            .long("output")
            .short('o')
            .value_name("FILE")
            .help("Write to FILE instead of STDERR"),
    )
    .arg(
        Arg::new("portability")
            .long("portability")
            .short('p')
            .action(ArgAction::SetTrue)
            .help("Print POSIX standard 1003.2 conformant string"),
    )
    .arg(
        Arg::new("quiet")
            .long("quiet")
            .short('q')
            .help(
                "Do not print information about abnormal program termination (non-zero exit codes or signals)",
            )
            .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("verbose")
            .long("verbose")
            .short('v').help("Print all resource usage information instead of the default format")
            .action(ArgAction::SetTrue),
    )
    .arg(Arg::new("command").required(true))
    .arg(Arg::new("arguments").action(ArgAction::Append))
}

fn main() {
    let matches = command().get_matches();

    let command: &str = matches.get_one::<String>("command").unwrap();
    let arguments: Vec<&str> = matches
        .get_many::<String>("arguments")
        .unwrap_or_default()
        .map(|v: &String| v.as_str())
        .collect();

    let result: time::ResourceUse = run_command(&command, arguments).unwrap();

    summarize(&result);
}

fn run_command(
    command: &str,
    arguments: Vec<&str>,
) -> Result<time::ResourceUse, Box<dyn error::Error>> {
    let mut resource_use: time::ResourceUse = time::ResourceUse::new();

    resource_use.begin();

    let _output: process::Output = process::Command::new(command).args(arguments).output()?;

    resource_use.finish();

    Ok(resource_use)
}

fn summarize(resource_use: &time::ResourceUse) {
    let elapsed = resource_use.elapsed();

    let mut elapsed =
        rust_decimal::Decimal::new(elapsed.as_millis() as i64, 3) / Decimal::new(1000, 3);

    elapsed.rescale(2);

    print!("real {}\nuser {:.2$}\nsys {:.2$}\n", elapsed, 0, 0);
}
