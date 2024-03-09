use std::io::{self, Write};
use std::process::Command;

const LSH_RL_BUFSIZE: usize = 1024;

fn lsh_read_line() -> String {
    let mut buffer = String::with_capacity(LSH_RL_BUFSIZE);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_string()
}

fn lsh_split_line(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

fn lsh_num_builtins() -> usize {
    ["cd", "help", "exit", "ls"].len()
}

fn lsh_cd(args: &[&str]) -> i32 {
    if args.len() < 2 {
        eprintln!("lsh: expected argument to \"cd\"");
    } else if let Err(err) = std::env::set_current_dir(args[1]) {
        eprintln!("lsh: {}", err);
    }
    1
}

fn lsh_help(_: &[&str]) -> i32 {
    println!("Rust Shell");
    println!("Enter program names and arguments, and press enter.");
    println!("Built-in commands:");
    println!("  cd <directory>: Change the current working directory");
    println!("  help: Display this help message");
    println!("  exit: Exit the shell");
    println!("  ls: List files and directories");
    1
}

fn lsh_exit(_: &[&str]) -> i32 {
    0
}

fn lsh_ls(_: &[&str]) -> i32 {
    let status = Command::new("ls").status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
            } else {
                eprintln!("ls command failed with exit code: {:?}", exit_status.code());
            }
        }
        Err(err) => {
            eprintln!("Error executing ls command: {}", err);
        }
    }
    1
}

fn lsh_execute(args: Vec<&str>) -> i32 {
    if args.is_empty() {
        return 1;
    }

    for i in 0..lsh_num_builtins() {
        if args[0] == ["cd", "help", "exit", "ls"][i] {
            return match i {
                0 => lsh_cd(&args),
                1 => lsh_help(&args),
                2 => lsh_exit(&args),
                3 => lsh_ls(&args),
                _ => 1,
            };
        }
    }

    lsh_launch(&args)
}

fn lsh_launch(args: &[&str]) -> i32 {
    let mut cmd = Command::new(args[0]);

    for arg in &args[1..] {
        cmd.arg(arg);
    }

    match cmd.spawn() {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(err) => {
            eprintln!("lsh: {}", err);
        }
    }

    1
}

fn lsh_loop() {
    loop {
        print!("rustShell > ");
        io::stdout().flush().unwrap();

        let line = lsh_read_line();
        let args = lsh_split_line(&line);
        let status = lsh_execute(args);

        if status == 0 {
            break;
        }
    }
}

fn main() {
    lsh_loop();
}
