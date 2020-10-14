pub mod branch;
pub mod member;
pub mod memberNode;

use crate::branch::BranchDB;
use crate::memberNode::{BranchMember, MemberManager, MemberNodeDB};
use mysql::*;
use std::io;
use std::time::{Duration, SystemTime};

pub fn init() {
    let connection_string = "mysql://root:HelloDev@192.168.2.53:3306/gmdatabase";
    let pool = Pool::new(connection_string).unwrap();
    let now = SystemTime::now();
    let now1 = SystemTime::now();
    let mut conn = pool.get_conn().unwrap();
    &conn.init();
    let user_branchs = &conn.get_user_branches();
    println!("分支 {:?}", user_branchs);
    let member_users = &conn.get_members(user_branchs);
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("读取用户 -> {} millis", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    for node in member_users {
        println!("{}-> 用户数{}", node.id, node.member.len());
    }
    //分支检索， id = u_branch_id
    let mut member_nodes: Vec<BranchMember> = vec![];
    &member_nodes.build_member(user_branchs, member_users);
    println!("load done");
    match now1.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("全部完成 {} secs", elapsed.as_secs());
            println!("全部完成 {} millis", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    &member_nodes.create_member("root", "xxxxxx", "fd9d906e-a7bb-4cf3-9987-16b4efa89308");
    &member_nodes.create_member("root", "aa1", "xxxxxx");
    &member_nodes.create_member("root", "aa2", "aa1");
    // member::print_node(x);//打印节点结构
    println!("test catalog user");
    let test = &member_nodes.catalog_user("root", "aa2");
    match test {
        Some(x) => {
            println!(
                "{} 's inviter = {}, department = {}",
                x.uid,
                x.inviter,
                x.invite_departments.read().unwrap().len()
            );
            // member::print_node(x);
        }
        None => println!("Can not find any member"),
    }

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
    }
}
