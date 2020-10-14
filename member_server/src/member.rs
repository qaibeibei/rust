trait IBaseManage {
    fn get_down_users();
    fn get_up_users();
    fn get_up_users_count() -> u32;
    fn get_down_users_count() -> u32;
}

struct BaseMemberManage {
    list: Vec<Rc<Member>>, //用户集合
}

impl BaseMemberManage {
    fn new() -> BaseMemberManage {
        BaseMemberManage { list: Vec::new() }
    }

    ///查询用户节点
    fn catalog_user(&self, branchid: String, otherid: String) -> Option<&Rc<Member>> {
        // let branch = self.list.iter().filter(|s| s.branch_id == branchid).next();
        // match branch {
        //     Some(x) => x.list.iter().filter(|s| s.user_id == otherid).next(),
        //     None => None,
        // }
        None
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

use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn init(){
    let mut basemanage = BaseMemberManage::new();

    let branch = Rc::new(Member {
        user_id: String::from("111"),
        user_name: String::from("111"),
        parent: RefCell::new(Weak::new()),
        childrens: RefCell::new(Vec::new()),
        // childrens: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    let leaf1 = Rc::new(Member {
        user_id: String::from("222"),
        user_name: String::from("222"),
        parent: RefCell::new(Weak::new()),
        childrens: RefCell::new(vec![]),
    });

    let leaf2 = Rc::new(Member {
        user_id: String::from("333"),
        user_name: String::from("333"),

        parent: RefCell::new(Weak::new()),
        childrens: RefCell::new(vec![]),
    });

    let leaf3 = Rc::new(Member {
        user_id: String::from("444"),
        user_name: String::from("444"),

        parent: RefCell::new(Weak::new()),
        childrens: RefCell::new(vec![]),
    });

    *leaf1.parent.borrow_mut() = Rc::downgrade(&branch);

    *leaf2.parent.borrow_mut() = Rc::downgrade(&branch);

    *leaf3.parent.borrow_mut() = Rc::downgrade(&leaf2);

    &basemanage.list.push(branch);

    println!("-------------------------------------------");
    println!("{:#?}", &basemanage.list);
    let a = &basemanage
        .list
        .iter()
        .filter(|s| s.user_id == String::from("111"))
        .next()
        .unwrap();

    let bbb = Rc::clone(&leaf1);

    a.childrens.borrow_mut().push(bbb);

    let leaf222 = Rc::clone(&leaf2);

    a.childrens.borrow_mut().push(leaf222);

    let leaf333 = Rc::clone(&leaf3);

    a.childrens.borrow_mut().push(leaf333);

    println!("basemanage.list -{:#?}", &basemanage.list);

}