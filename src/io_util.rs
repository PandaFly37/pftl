use crate::Todo;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use std::{
    fs,
    io::{Write, stdin, stdout},
};

pub fn load_todos() -> (Vec<Todo>, i32) {
    match fs::read_to_string("todos.json") {
        Ok(content) => {
            let todos = serde_json::from_str(&content).unwrap_or_default();
            let mut todo_cnt = 0;
            for todo in &todos {
                let td: &Todo = todo;
                if td.id > todo_cnt {
                    todo_cnt = todo.id;
                }
            }
            (todos, todo_cnt)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                (Vec::new(), 0)
            } else {
                eprintln!("读取文件失败：{}", e);
                (Vec::new(), 0)
            }
        }
    }
}

pub fn save_todos(todos: &[Todo]) {
    let text = serde_json::to_string_pretty(todos).expect("序列化失败");
    fs::write("todos.json", text).expect("写入文件失败");
}

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
            eprintln!("输入时发生错误: {}", e);
            ()
        }
    };
    inp
}

pub fn print_page(msg: &str, options: &[&str]) -> String {
    clear_screen();
    // 打印内容
    println!("PandaFly_37's Todo List (v0.1.0)\n");
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
