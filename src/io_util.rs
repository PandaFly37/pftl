use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdin, stdout};

pub fn clear_screen() {
    match execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)) {
        Ok(_) => (),
        Err(_) => {
            for _ in 0..20 {
                print!("\n")
            }
        }
    }
}

pub fn input_line() -> String {
    let mut inp = String::new();
    match stdin().read_line(&mut inp) {
        Ok(_) => (),
        Err(e) => {
            println!("输入时发生错误: {}", e);
            ()
        }
    };
    inp
}

pub fn print_page(pftl: bool, msg: &str, options: &[&str]) -> String {
    clear_screen();
    // 打印内容
    if pftl {
        println!("PandaFly_37's Todo List\n");
    } else {
        println!("PandaFly_37's Todo\n");
    }
    println!("{}", msg);
    println!();
    for i in options {
        println!("> {}", i);
    }
    print!("==========\n> ");
    // 输入
    stdout().flush().unwrap();
    input_line().trim().to_string()
}
