use std::fs::File;
use std::io::{Read, Write};

fn fwrite(file_path:&str, input_args:Vec<String> ) { 
    let mut todo_file = File::create(&file_path).expect("create failed");
    todo_file.write(&input_args[0].as_bytes()).expect("unable to write");  
    //println!("Hello, world! {:?}", args[1]);
}

fn read(file_path:&str) -> String {
    let mut todo_opened = File::open(&file_path).expect("Unable to open");
         
    let mut todo_contents:String = String::new();
    todo_opened.read_to_string(&mut todo_contents).expect("failed to read file");
    todo_contents
}

fn print_todo(file_path:&str) {
    let file_path:String = read(&file_path);
    let todo_vec:Vec<&str> = file_path.split("\n").collect();
    let mut num:u8 = 1;
    for i in todo_vec.iter() {
        if i.len() > 0 {
            println!("{:>2}. {}", num, i);
            num += 1;
        }
    }
}

fn main() {
    let file:&str = "/home/sean/Desktop/.todo";
    let mut args:Vec<String> = std::env::args().collect();

    // if there is nothing to add, print out the current todo
    if args.len() == 1 {
        //let mut todos = &read(&file).split("\n");
        print_todo(&file);     
        std::process::exit(0);
    }

    args.remove(0);

    if args[0] == "rm" {
        let mut split_content:Vec<String> = Vec::with_capacity(0); 
            
        let file_path:String = read(&file);
        let old_content:Vec<&str> = file_path.split("\n").collect();

        for i in old_content {
            split_content.push(String::from(i));
        }

        let x:u8 = args[1].parse().unwrap();

        split_content.remove((x-1) as usize);
 
        let new_todo:Vec<String> = vec![split_content.join("\n")];

        fwrite(&file, new_todo);
        print_todo(&file);

        std::process::exit(0);
    } 
    let mut new_contents:Vec<String> = Vec::with_capacity(0);
    //let new_content:String = args.join(" "); 
    new_contents.push(read(&file) + "\n" + &args.join(" ")); 
    fwrite(&file, new_contents); 

    print_todo(&file);

    std::process::exit(0);
}
