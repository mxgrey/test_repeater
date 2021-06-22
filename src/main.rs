use std::process::Command;
use argparse::{ArgumentParser, Store, Collect};
use std::io::{self, Write};


fn main() {

    let mut command = String::new();
    let mut args = Vec::<String>::new();
    let mut cycles = 100u32;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Repeat a test many times and report failures");

        parser.refer(&mut cycles)
            .add_option(&["-c", "--cycles"], Store, "Number of cycles to run");

        parser.refer(&mut command)
            .add_argument("command", Store, "Executable name");

        parser.refer(&mut args)
            .add_argument("args", Collect, "Arguments for the executable");

        parser.parse_args_or_exit();
    }

    let mut failed = 0u32;
    for i in 1..=cycles {
        println!("Running {}/{}", i, cycles);

        let test_output = Command::new(command.clone())
            .args(args.clone())
            .output()
            .expect(&format!("Failed to start {}", command));

        if !test_output.status.success() {
            failed += 1u32;
            println!(" ------------------------------------------ ");
            println!("Failure count: {}", failed);
            io::stdout().write_all(&test_output.stdout).unwrap();
        }
    }

    println!("Total failures: {}", failed);
}
