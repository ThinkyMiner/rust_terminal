use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
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
    ["cd", "help", "exit", "ls", "mkdir", "cp", "touch", "cat"].len()
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


fn lsh_mkdir(arguments: &[&str]) -> i32 {
    if arguments.len() > 1 {
        println!("Too many arguments.");
    } else if arguments.len() < 1 {
        println!("Directory name not found.");
    } else {
        let dir_name = arguments[0];
        
        match fs::create_dir(dir_name) {
            Ok(_) => println!("Directory created successfully"),
            Err(err) => {
                eprintln!("Error executing mkdir command: {}", err);
            }
        }
    }
    1
}


fn lsh_cat(file_path: &[&str]) -> i32 {
    let path = Path::new(file_path[1]);

    if !path.exists() {
        eprintln!("Error: File '{}' does not exist.", file_path[0]);
        return 1;
    }
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) => {
            match error.kind() {
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Error: Permission denied to read file '{}'.", file_path[0]);
                }
                _ => {
                    eprintln!(
                        "An unexpected error occurred while reading file '{}': {}",
                        file_path[0], error
                    );
                }
            }
            return 1;
        }
    };

    println!("{}", contents);
    1
}

fn lsh_cp(arguments: &[&str]) -> i32 {
    if arguments.len() != 3 {
        println!("Usage: cp <source_file> <destination_file>");
        return 1;
    }

    let source_path = arguments[1];
    let destination_path = arguments[2];

    let source_contents = match fs::read_to_string(source_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading source file '{}': {}", source_path, err);
            return 1;
        }
    };

    match fs::write(destination_path, source_contents) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error writing to destination file '{}': {}", destination_path, err);
            1
        }
    }
}

fn lsh_touch(file_path: &[&str]) -> i32 {
    if file_path.len() < 2 {
        eprintln!("Too few arguments");
        return 1;
    }
    else if file_path.len() > 2 {
        eprintln!("Too many arguments");
        return 1;
    }
    else{
        let file_result = File::create(file_path[1]);

        match file_result {
            Ok(_) => {
                println!("File created successfully: {}", file_path[1]);
                1
            }
            Err(err) => {
                eprintln!("Error creating file '{}': {}", file_path[0], err);
                1
            }
        }
    }
}


fn lsh_execute(args: Vec<&str>) -> i32 {
    if args.is_empty() {
        return 1;
    }

    let builtins = ["cd", "help", "exit", "ls", "cat", "mkdir", "cp", "touch"];

    for i in 0..lsh_num_builtins() {
        if args[0] == builtins[i] {
            return match i {
                0 => lsh_cd(&args),
                1 => lsh_help(&args),
                2 => lsh_exit(&args),
                3 => lsh_ls(&args),
                4 => lsh_cat(&args),
                5 => lsh_mkdir(&args),
                6 => lsh_cp(&args),
                7 => lsh_touch(&args),
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
