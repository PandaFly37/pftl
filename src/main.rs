mod io_util;

use io_util::*;
use std::fs;
use std::io::{BufRead, BufReader, Write, stdin, stdout};
use std::process;

fn parse_input(op: &str) -> Option<(char, Option<usize>)> {
    let s = op.trim();
    if s.is_empty() {
        return None;
    }

    let parts = s.split_whitespace().collect::<Vec<_>>();
    match parts.len() {
        2 => {
            if parts[0].len() != 1 {
                None
            } else {
                Some((
                    parts[0].chars().next().unwrap(),
                    match parts[1].parse::<usize>() {
                        Ok(n) => Some(n),
                        Err(_) => return None,
                    },
                ))
            }
        }
        1 => {
            if parts[0].len() != 1 {
                None
            } else {
                Some((parts[0].chars().next().unwrap(), None))
            }
        }
        _ => None,
    }
}

fn main() {
    let mut todos: Vec<String> = Vec::new();
    {
        println!("打开TODO文件…");
        let file = match fs::File::open("todo.txt") {
            Ok(f) => f,
            Err(e) => {
                print_page(true, &format!("打开TODO文件时出错: {e}"), &["[any] 退出"]);
                process::exit(1);
            }
        };
        println!("读取TODO文件…");
        let reader = BufReader::new(file);
        let mut now_line = 1;
        for i in reader.lines() {
            match i {
                Ok(i_text) => {
                    todos.push(i_text);
                    now_line += 1;
                }
                Err(e) => {
                    print_page(
                        true,
                        &format!("(test)读取TODO文件内容时出错.\n行数: {now_line}\n错误: {e}"),
                        &["[Any] 退出"],
                    );
                    process::exit(1);
                }
            }
        }
    }
    let mut next_page = 'm';
    // next_page
    // m 主页
    // n 新建
    // d 确认删除
    // v 详情
    // u 确认设为未完成
    let mut next_todo_id = 0;
    loop {
        match next_page {
            'm' => {
                let mut msg = String::from("TODO\n");
                for i in 0usize..todos.len() {
                    msg.push_str((i + 1).to_string().as_str());
                    msg.push_str(" | ");
                    msg.push_str(todos[i].as_str());
                    msg.push_str("\n");
                }
                let op = print_page(
                    true,
                    &msg,
                    &[
                        "[n] 新建TODO",
                        "[v todo_id] 查看TODO",
                        "[d todo_id] 完成TODO",
                        "[e] 退出",
                    ],
                );
                let parsed_op = parse_input(&op);
                println!("{:?}", parsed_op);
            }
            _ => {
                print!("无效页面.\n> ");
                stdout().flush().unwrap();
                let op = input_line().trim().to_string();
                println!("{}", op);
            }
        }
    }
}
