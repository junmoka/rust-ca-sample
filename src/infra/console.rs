pub mod dto;

use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::infra::console::dto::create_todo::CTodoInput;
use crate::infra::db::kvs::KvsImpl;
use crate::adapters::controllers::create_todo::CreateTodoController;
use crate::adapters::controllers::show_todo::ShowTodoController;
use crate::adapters::repositories::todo::TodoRepositoryImpl;
//use crate::adapters::dto::create_todo;

#[allow(dead_code)]
pub fn run(){
    println!("Run console");

    loop{
        let line: String = read();
        let cmd: Vec<&str> = line.split(' ').collect();
        println!("{:?}", cmd);

        if cmd.len() == 1 && cmd[0] == "exit"{
            return
        }
        else if 1 < cmd.len() && cmd[0] == "todo" {
            match cmd[1] {
                "add" => create_todo(cmd[2]),
                "show" => show_todo(),
                _ =>  println!("not cmd"),
            }
        }
        else {
            println!("{}", line);
        }


    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn create_todo(arg: &str){
    let name = arg.to_string();
    let controller = CreateTodoController::new(Box::new(TodoRepositoryImpl::new(Box::new(KvsImpl{}))));
    controller.create(name);
}

fn show_todo(){
    let controller = ShowTodoController::new(Box::new(TodoRepositoryImpl::new(Box::new(KvsImpl{}))));
    controller.show();
}