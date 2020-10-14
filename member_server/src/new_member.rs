trait IBaseManage {
    // fn build();
    fn get_down_users();
    fn get_up_users();
    fn get_up_users_count() -> u32;
    fn get_down_users_count() -> u32;
}

pub struct BaseMemberManage {
    list: Vec<Rc<Dic>>, //用户集合
}

impl BaseMemberManage {
    fn new() -> Self {
        BaseMemberManage { list: Vec::new() }
    }

    ///查询用户节点
    fn catalog_user(&self, branchid: String, userid: String) -> Option<&Rc<Member>> {
        let branch = self.list.iter().filter(|s| s.branch_id == branchid).next();
        match branch {
            Some(x) => x.list.iter().filter(|s| s.user_id == userid).next(),
            None => None,
        }
    }

    //添加子节点
    fn add_user(&self, branchid: String, member: Rc<Member>) -> bool {
        let branch = self.list.iter().filter(|s| s.branch_id == branchid).next();

        if let Some(x) = branch {
            let user = x.list.iter().filter(|s| s.user_id == member.user_id).next();
            if let None = user {
                // x.list.push(member);
            }
        }
        false
    }
}

impl IBaseManage for BaseMemberManage {
    fn get_down_users() {
        println!("get_down_users");
    }
    fn get_down_users_count() -> u32 {
        1
    }
    fn get_up_users() {
        println!("get_up_users");
    }
    fn get_up_users_count() -> u32 {
        1
    }
}

#[derive(Debug)]
struct Dic {
    branch_id: String,
    list: Vec<Rc<Member>>,
}

#[derive(Debug)]
struct Member {
    user_id: String,
    user_name: String,
    parent: RefCell<Weak<Member>>,
    childrens: RefCell<Vec<Rc<Member>>>,
}

impl Member {
    ///添加邀请下线
    fn add_invite_child(&self, member: Rc<Member>) {
        self.childrens.borrow_mut().push(member);
    }
    ///添加安置下线
    fn add_settler_child(&self, member: Member) {}
    ///设置安置
    fn set_settler(&self, member: Member) {}
    ///设置邀请
    fn set_inviter(&self, member: &Rc<Member>) {
        *self.parent.borrow_mut() = Rc::downgrade(member);
    }
}

use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct UserBranch {
    branch_id: String,
    user_id: String,
    user_name: String,
}

///初始化baseMemberManage
pub fn init() -> BaseMemberManage {
    let mut list_branch = Vec::new();
    list_branch.push(UserBranch {
        branch_id: String::from("system"),
        user_id: String::from("root"),
        user_name: String::from("root"),
    });

    list_branch.push(UserBranch {
        branch_id: String::from("mall"),
        user_id: String::from("root"),
        user_name: String::from("root"),
    });

    let mut basemanage = BaseMemberManage::new();

    //加载 branch 用户
    for bran in list_branch {
        let branch_mem = Rc::new(Member {
            user_id: bran.user_id,
            user_name: bran.user_name,
            parent: RefCell::new(Weak::new()),
            childrens: RefCell::new(Vec::new()),
            // childrens: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        let branch = Rc::new(Dic {
            branch_id: bran.branch_id,
            list: [branch_mem].to_vec(),
        });

        &basemanage.list.push(branch);
    }

    let roo = basemanage.catalog_user(String::from("system"), String::from("root"));

    for i in 1..=10 {
        let user = basemanage.catalog_user(String::from("system"), i.to_string());

        //添加用户
        if let Some(r) = roo {
            if let None = user {
                let newuser = Rc::new(Member {
                    user_id: i.to_string(),
                    user_name: i.to_string(),
                    parent: RefCell::new(Weak::new()),
                    childrens: RefCell::new(Vec::new()),
                });

                newuser.set_inviter(&r);

                // *newuser.parent.borrow_mut() = Rc::downgrade(&r);

                // println!("{:#?}", newuser.parent.borrow_mut().upgrade());

                r.add_invite_child(newuser.clone());
            }
        }
    }

    println!("basemanage.list - {:#?}", &basemanage.list);

    basemanage
}
