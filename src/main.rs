use std::fs;
use std::io::{stdin, stdout, Write, BufRead, BufReader};
use std::process;
use crossterm::{
    terminal::{Clear, ClearType},
    cursor,
    execute
};

fn clear_screen() {
    match execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0,0)) {
        Ok(_) => (),
        Err(_) => for _ in 0..20 {
            print!("\n")
        }
    }
}

fn print_page(pftl: bool, msg: &str, options: &[&str]) -> String {
    clear_screen();
    // 打印内容
    if pftl {
        println!("PandaFly_37's Todo List\n");
    } else {
        println!("PandaFly_37's Todo\n");
    }
    println!("{}", msg);
    println!("");
    for i in options {
        println!("> {}", i);
    }
    print!("==========\n> ");
    // 输入
    stdout().flush().unwrap();
    let mut inp = String::new();
    match stdin().read_line(&mut inp) {
        Ok(_) => (),
        Err(e) => {
            println!("输入时发生错误: {}", e);
            ()
        }
    }
    inp.trim().to_string()
}

fn main() {
    let file = match fs::File::open("todo.txt") {
        Ok(f) => f,
        Err(e) => {
            print_page(true, &format!("打开TODO文件时出错: {e}"), &["[any] 退出"]);
            process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    for i in reader.lines() {
        match i {
            Ok(i_text) => println!("{}", i_text),
            Err(e) => {
                print_page(true, &format!("(test)输出TODO文件内容时出错: {e}"), &["[any] 退出"]);
                process::exit(1);
            }
        }
    }
}