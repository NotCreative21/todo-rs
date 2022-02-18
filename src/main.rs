use std::env;
use std::fs::{read_to_string, remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use dirs::home_dir;
use std::fs::OpenOptions;

fn fwrite(file_path: &str, input_args: Vec<String>) -> std::io::Result<()> { 
    let mut todo_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    todo_file.write_all(
            ("\n".to_owned() + &input_args.join(" ")
         ).as_bytes())?;
    Ok(())
}

fn print_todo(file_path: &str) -> std::io::Result<()> {
    let file_path: String = read_to_string(file_path)?;
    let todo_vec: Vec<&str> = file_path.split("\n").collect();
    let mut num: u8 = 1;
    for i in todo_vec.iter() {
        if i.len() > 0 {
            println!("{:>2}. {}", num, i);
            num += 1;
        }
    }
    Ok(())
}

fn get_data(file_path: &str) -> Vec<String> {
    let data = match read_to_string(file_path) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("could not read todo");
            std::process::exit(1);
        },
    };
    let mut values = Vec::new();
    data.split("\n").for_each(|x| values.push(x.to_string()));
    values
}

fn write_replace(file_path: &str, contents: String) -> std::io::Result<()> {
    let mut todo = match OpenOptions::new()
        .write(true)
        .open(file_path) 
    {
        Ok(v) => v,
        Err(_) => {
            eprintln!("failed to open todo list, creating file");
            let _ = File::create(file_path);
            OpenOptions::new()
                .write(true)
                .open(file_path)
                .expect("failed to open file")
        }
    };
    let contents = contents.as_bytes();
    todo.set_len(contents.len() as u64)?;
    todo.write_all(contents)?;
    Ok(())
}

fn parse_range(arg: String) -> (usize, usize) {
    if arg.len() < 3 { std::process::exit(1); }

    let nums: Vec<&str> = arg.split("-").collect();

    let result = match (nums[0].parse(), nums[1].parse()) {
        (Ok(v), Ok(e)) => (v, e),
        _ => {
            eprintln!("invalid range!");
            std::process::exit(1);
        }
    };

    result
}

fn remove_data(file: &str, removal: usize) -> std::io::Result<()> {
    let mut data = get_data(file);
    if removal < 1 || removal > data.len() {
        eprintln!("invalid length to remove");
        std::process::exit(1); 
    }
    data.remove(removal);

    write_replace(file, data.join("\n"))?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let file: &str = &(
        home_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned() + 
            "/.config/todo"
        );
    let file = match PathBuf::from("./todo").exists() {
        true => "./todo",
        false => file
    };
    let args: Vec<String> = env::args().skip(1).collect();

    // if there is nothing to add, print out the current todo
    if args.len() == 0 {
        print_todo(&file)?;
        std::process::exit(0);
    }

    match args[0].as_str() {
        "rm" => {
            if args.len() < 2 { std::process::exit(1); }
            let removal: usize = match args[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    match args[1].as_str() {
                        "all" => {
                            remove_file(file)?;
                            println!("removed todo in current directory");
                            std::process::exit(0);
                        },
                        _ => {
                            let (start, end) = parse_range(args[1].clone());

                            let range = end - start;
                            let mut data = get_data(file);
                            for _ in 0..(range + 1) {
                                data.remove(start);
                            }
                            write_replace(file, data.join("\n"))?;
                            std::process::exit(0);
                        }
                    }
                }
            };
            remove_data(file, removal)?;
        },
        "new" => {
            File::create(file)?;
            println!("created new todo file at {}", file);
        }
        _ => fwrite(&file, args)? 
    }
    Ok(())
}
