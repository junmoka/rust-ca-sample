use crate::infra::adapters::controllers::prelude::*;
use crate::infra::adapters::usecase_bus::UsecaseBus;

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
    let controller = TodoController::new(UsecaseBus{});
    controller.create(name);
}

fn show_todo(){
    let controller = TodoController::new(UsecaseBus{});
    controller.show();
}