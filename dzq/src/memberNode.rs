use crate::branch::Branch;
use crate::member::{Member, MemberUpdate, RefMember};
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::{Arc, RwLock, Weak};
use std::thread;
use std::time::{Duration, SystemTime};
///分支 member
pub struct BranchMember {
    ///u_branch_id
    pub id: String,
    /// u_branch_id的用户
    pub member: Vec<RefMember>,
}

trait BranchManager {
    ///Branch插入多个RefMember
    fn insert_branchs(&mut self, members: &Vec<RefMember>, u_branch_id: &str);
    ///Branch插入一个节点RefMember
    fn insert_init_branch(&mut self, member: &RefMember, u_branch_id: &str);
    ///分支初始化
    fn init_branchs(&mut self, branchs: &Vec<Branch>);
    ///节点用户
    fn root_user(&self, u_branch_id: &str) -> Option<RefMember>;
}
impl BranchManager for Vec<BranchMember> {
    ///Branch插入多个RefMember
    fn insert_branchs(&mut self, members: &Vec<RefMember>, u_branch_id: &str) {
        for item in members {
            self.insert_init_branch(item, &u_branch_id);
        }
    }
    ///Branch插入一个节点RefMember
    fn insert_init_branch(&mut self, member: &RefMember, u_branch_id: &str) {
        let branch = self.into_par_iter().find_any(|x| x.id == u_branch_id);
        match branch {
            Some(x) => {
                x.member.push(member.clone()); //已经存在
            }
            None => {
                let mut new_node = BranchMember {
                    id: String::from(u_branch_id),
                    member: Vec::new(),
                };
                new_node.member.push(member.clone());
                self.push(new_node)
            }
        }
    }
    ///分支初始化
    fn init_branchs(&mut self, branchs: &Vec<Branch>) {
        for branch in branchs {
            let member = Member::new(&branch.uid, &branch.u_branch_id, &branch.branch_id, "");
            let new_member = Arc::new(member);
            self.insert_init_branch(&new_member, &branch.u_branch_id);
        }
    }

    ///节点用户
    fn root_user(&self, u_branch_id: &str) -> Option<RefMember> {
        let branch = self.par_iter().find_any(|x| x.id == u_branch_id);
        match branch {
            Some(x) => {
                let m = &x.member[0]; //.clone();
                Some(Arc::clone(m))
            }
            None => None,
        }
    }
}
pub trait MemberManager {
    ///检索用户
    fn catalog_user(&self, u_branch_id: &str, uid: &str) -> Option<RefMember>;
    ///增加一个用户
    fn create_member(&mut self, u_branch_id: &str, uid: &str, inviter: &str);
    //构建
    fn build_member(&mut self, branchs: &Vec<Branch>, member_users: &Vec<BranchMember>);
}

//self Branchs
impl MemberManager for Vec<BranchMember> {
    ///检索用户
    fn catalog_user(&self, u_branch_id: &str, uid: &str) -> Option<RefMember> {
        let branch = self.par_iter().find_any(|x| x.id == u_branch_id);
        match branch {
            Some(x) => {
                let um = (&x.member).par_iter().find_any(|x| x.uid == uid);
                match um {
                    Some(y) => Some(Arc::clone(y)),
                    None => {
                        println!("{} in member is empty", uid);
                        None
                    }
                }
            }
            None => {
                println!("{} in node is empty", u_branch_id);
                None
            }
        }
    }
    ///增加一个用户
    fn create_member(&mut self, u_branch_id: &str, uid: &str, inviter: &str) {
        let inviter_member = self.catalog_user(u_branch_id, inviter);
        match inviter_member {
            Some(x) => {
                let user = Arc::new(Member::new(uid, u_branch_id, &x.branch_id, inviter));
                x.and_invite_child(&user);
                self.insert_init_branch(&user, &u_branch_id);
            }
            None => {
                println!("catalog_user is empty");
                //Err
            }
        }
    }
    //构建
    //这里耗时间，需要优化
    fn build_member(&mut self, branchs: &Vec<Branch>, member_users: &Vec<BranchMember>) {
        self.init_branchs(branchs);
        for node in member_users {
            //分支用户
            let root = &self.root_user(&node.id); //节点用户
            match root {
                Some(x) => {
                    let users: &Vec<RefMember> = &node.member;
                    //根据邀请人
                    let mut group_users: Vec<UserMember> = vec![];
                    for (key, group) in &users
                        .into_iter()
                        .sorted_by_key(|k| &k.inviter)
                        .group_by(|elt| &elt.inviter)
                    {
                        let mut new_node = UserMember {
                            uid: String::from(key),
                            member: Vec::new(),
                        };
                        for g in group.into_iter() {
                            new_node.member.push(g.clone());
                        }
                        group_users.push(new_node);
                    }
                    let now = SystemTime::now();
                    fetch_next_inviter(x, &group_users);
                    match now.elapsed() {
                        Ok(elapsed) => {
                            println!("fetch_next_inviter -> {} millis", elapsed.as_millis());
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                        }
                    }
                    //member::print_node(x);//打印节点结构
                }
                None => {
                    //TODO
                    println!("can not find any root_user in {}", node.id);
                }
            }
        }
        for user in member_users {
            self.insert_branchs(&user.member, &user.id);
        }
    }
}
//用户加载时 建立结构缓存用 groupby inviter时
struct UserMember {
    ///uid
    uid: String,
    /// u_branch_id的用户
    member: Vec<RefMember>,
}

///迭代推荐关系
fn fetch_next_inviter(member: &RefMember, group_users: &Vec<UserMember>) {
    //group_user id = uid (邀请人/安置人)
    let child = group_users.par_iter().find_any(|x| x.uid == member.uid);
    match child {
        Some(x) => {
            x.member.par_iter().for_each(|x| {
                member.and_invite_child(&x);
                fetch_next_inviter(&x, group_users);
            });
            // for item in &x.member{
            //     member.and_invite_child(&item);
            //     fetch_next_inviter(&item,group_users);
            // }
        }
        None => {}
    }
}

use mysql::prelude::*;
use mysql::*;
pub trait MemberNodeDB {
    //获取当前存在的用户
    fn get_members(&mut self, branches: &Vec<Branch>) -> Vec<BranchMember>;
}
impl MemberNodeDB for PooledConn {
    fn get_members(&mut self, branches: &Vec<Branch>) -> Vec<BranchMember> {
        let mut group_branch_users: Vec<BranchMember> = vec![];
        for item in branches {
            let members: Vec<RefMember> = self
                .query_map(
                    format!(
                        "SELECT uid, u_branch_id, inviter FROM {}_Ship",
                        &item.branch_id
                    ),
                    |(uid, u_branch_id, inviter)| {
                        Arc::new(Member {
                            uid,
                            u_branch_id,
                            inviter,
                            branch_id: item.u_branch_id.clone(),
                            inviter_prev: RwLock::new(Weak::new()),
                            invite_departments: RwLock::new(vec![]),
                            invite_index: RwLock::new(0),
                        })
                    },
                )
                .unwrap();
            group_branch_users.push(BranchMember {
                id: item.u_branch_id.clone(),
                member: members,
            })
        }
        group_branch_users
    }
}
