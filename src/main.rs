mod io_util;

use io_util::*;
use serde::{Deserialize, Serialize};
use std::io::{Write, stdout};
use std::process;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    name: String,
    done: bool,
}

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
    let mut todos = load_todos();
    let mut next_page = 'm';
    // next_page
    // m 主页
    // n 新建
    // r 确认删除
    // v 详情
    // u 确认设为未完成
    let mut next_todo_id = 0;
    loop {
        match next_page {
            'm' => {
                let mut msg = String::from("TODO\n");
                for i in 0usize..todos.len() {
                    msg.push_str((i + 1).to_string().as_str());
                    msg.push_str(if todos[i].done { " - 已完成" } else { "" });
                    msg.push_str(" | ");
                    msg.push_str(todos[i].name.as_str());
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
                match parsed_op {
                    Some(('n', _)) => next_page = 'n',
                    Some(('v', Some(id))) => {
                        next_page = 'v';
                        next_todo_id = id;
                    }
                    Some(('d', Some(id))) => {
                        let todo_idx = id - 1;
                        if todo_idx < todos.len() {
                            todos[todo_idx].done = true;
                        }
                    }
                    Some(('e', _)) => {
                        save_todos(&todos);
                        process::exit(0)
                    }
                    _ => (),
                }
            }
            'n' => {
                let new_todo_name =
                    print_page(false, "新建TODO\n注意: TODO名不能为空", &["输入TODO名称"]);
                if new_todo_name.len() == 0 {
                    print_page(false, "TODO名不能为空.", &["[b] 返回主页"]);
                    next_page = 'm';
                } else {
                    todos.push(Todo {
                        name: new_todo_name.clone(),
                        done: false,
                    });
                    let op = print_page(
                        false,
                        &format!("创建TODO\"{new_todo_name}\"成功."),
                        &["[b] 返回主页", "[v] 查看详情"],
                    );
                    let parsed_op = parse_input(&op);
                    match parsed_op {
                        Some(('b', _)) => next_page = 'm',
                        Some(('v', _)) => {
                            next_page = 'v';
                            next_todo_id = todos.len();
                        }
                        _ => next_page = 'm',
                    }
                }
            }
            'r' => next_page = 'm', // WIP
            'v' => {
                let todo_idx = next_todo_id - 1;
                if todo_idx < todos.len() {
                    let todo_name = &todos[todo_idx].name;
                    let todo_done_tag = if todos[todo_idx].done {
                        " - 已完成"
                    } else {
                        ""
                    };
                    let op = print_page(
                        false,
                        &format!("TODO {next_todo_id}{todo_done_tag}\n{todo_name}"),
                        &[
                            "[b] 返回主页",
                            if todos[todo_idx].done {
                                "[u] 设为未完成"
                            } else {
                                "[d] 完成"
                            },
                            "[r] 删除",
                        ],
                    );
                    let parsed_op = parse_input(&op);
                    match parsed_op {
                        Some(('b', _)) => next_page = 'm',
                        Some(('d', _)) => todos[todo_idx].done = true,
                        Some(('r', _)) => next_page = 'r',
                        _ => (),
                    }
                } else {
                    print_page(false, "todo_id无效.", &["[b] 返回主页"]);
                    next_page = 'm';
                };
            }
            'u' => next_page = 'm', // WIP
            _ => {
                eprint!("无效页面.\n> ");
                stdout().flush().unwrap();
                let op = input_line().trim().to_string();
                println!("{}", op);
            }
        }
    }
}
