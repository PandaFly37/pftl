mod io_util;

use io_util::*;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    name: String,
    done: bool,
}

enum Page {
    Main,
    New,
    Remove(usize),
    View(usize),
    Undone(usize),
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
    let mut next_page = Page::Main;
    loop {
        match next_page {
            Page::Main => {
                let mut msg = String::from("TODO\n");
                for i in 0usize..todos.len() {
                    msg.push_str((i + 1).to_string().as_str());
                    msg.push_str(if todos[i].done { " - 已完成" } else { "" });
                    msg.push_str(" | ");
                    msg.push_str(todos[i].name.as_str());
                    msg.push_str("\n");
                }
                let op = print_page(
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
                    Some(('n', _)) => next_page = Page::New,
                    Some(('v', Some(id))) => next_page = Page::View(id),
                    Some(('d', Some(id))) => {
                        let todo_idx = id - 1;
                        if todo_idx < todos.len() {
                            todos[todo_idx].done = true;
                            save_todos(&todos);
                        } else {
                            print_page("todo_id无效.", &["[b] 返回主页"]);
                        }
                    }
                    Some(('e', _)) => {
                        save_todos(&todos);
                        process::exit(0);
                    }
                    _ => (),
                }
            }
            Page::New => {
                let new_todo_name = print_page("新建TODO\n注意: TODO名不能为空", &["输入TODO名称"]);
                if new_todo_name.len() == 0 {
                    print_page("TODO名不能为空.", &["[b] 返回主页"]);
                    next_page = Page::Main;
                } else {
                    todos.push(Todo {
                        name: new_todo_name.clone(),
                        done: false,
                    });
                    save_todos(&todos);
                    let op = print_page(
                        &format!("创建TODO\"{new_todo_name}\"成功."),
                        &["[b] 返回主页", "[v] 查看详情"],
                    );
                    let parsed_op = parse_input(&op);
                    match parsed_op {
                        Some(('b', _)) => next_page = Page::Main,
                        Some(('v', _)) => next_page = Page::View(todos.len()),
                        _ => next_page = Page::Main,
                    }
                }
            }
            Page::Remove(next_todo_id) => {
                let todo_idx = next_todo_id - 1;
                if todo_idx < todos.len() {
                    let todo_name = &todos[todo_idx].name;
                    match parse_input(&print_page(
                        &format!("确定要删除TODO\"{todo_name}\"吗?\n此操作不可撤销!"),
                        &["[y] 是", "[n] 否"],
                    )) {
                        Some(('y', _)) => {
                            todos.remove(todo_idx);
                            save_todos(&todos);
                            next_page = Page::Main;
                        }
                        _ => next_page = Page::View(next_todo_id),
                    }
                } else {
                    print_page("todo_id无效.", &["[b] 返回主页"]);
                    next_page = Page::Main;
                };
            }
            Page::View(next_todo_id) => {
                let todo_idx = next_todo_id - 1;
                if todo_idx < todos.len() {
                    let todo_name = &todos[todo_idx].name;
                    let todo_done_tag = if todos[todo_idx].done {
                        " - 已完成"
                    } else {
                        ""
                    };
                    let op = print_page(
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
                        Some(('b', _)) => next_page = Page::Main,
                        Some(('d', _)) => {
                            todos[todo_idx].done = true;
                            save_todos(&todos);
                        }
                        Some(('u', _)) => next_page = Page::Undone(next_todo_id),
                        Some(('r', _)) => next_page = Page::Remove(next_todo_id),
                        _ => (),
                    }
                } else {
                    print_page("todo_id无效.", &["[b] 返回主页"]);
                    next_page = Page::Main;
                };
            }
            Page::Undone(next_todo_id) => {
                let todo_idx = next_todo_id - 1;
                if todo_idx < todos.len() {
                    if !todos[todo_idx].done {
                        print_page("该TODO尚未完成.", &["[b] 返回详情页"]);
                    } else {
                        let todo_name = &todos[todo_idx].name;
                        match parse_input(&print_page(
                            &format!("确定要设置TODO\"{todo_name}\"为未完成吗?"),
                            &["[y] 是", "[n] 否"],
                        )) {
                            Some(('y', _)) => {
                                todos[todo_idx].done = false;
                                save_todos(&todos);
                            }
                            _ => (),
                        }
                    }
                    next_page = Page::View(next_todo_id);
                } else {
                    print_page("todo_id无效.", &["[b] 返回主页"]);
                    next_page = Page::Main;
                };
            }
        }
    }
}
