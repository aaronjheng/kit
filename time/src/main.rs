extern crate libc;

use std::error;
use std::ffi;
use std::process;
use std::ptr;

use clap::{crate_name, crate_version, Arg, ArgAction, Command};
// use libc;
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

    let pid = unsafe { libc::fork() };

    // Child
    if pid == 0 {
        let mut c_args = Vec::with_capacity(arguments.len() + 1);
        c_args.push(command);
        c_args.extend_from_slice(&arguments);

        let command = ffi::CString::new(command).unwrap();

        let args_cstr: Vec<ffi::CString> = c_args
            .iter()
            .map(|&arg| ffi::CString::new(arg).expect("CString creation failed"))
            .collect();

        let args_ptrs: Vec<*const ffi::c_char> =
            args_cstr.iter().map(|cstr| cstr.as_ptr()).collect();

        let mut args_ptrs_with_null: Vec<*const ffi::c_char> = args_ptrs.clone();
        args_ptrs_with_null.push(ptr::null());

        let result = unsafe { libc::execvp(command.as_ptr(), args_ptrs_with_null.as_ptr()) };
        if result == -1 {
            println!("Failed")
        }

        process::exit(0);
    }

    let mut status: libc::c_int = 0;
    let options: libc::c_int = 0; // No options
    let mut rusage: libc::rusage = libc::rusage {
        ru_utime: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    };

    let _pid = unsafe { libc::wait4(pid, &mut status, options, &mut rusage) };

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
