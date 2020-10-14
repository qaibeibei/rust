
use std::sync::{RwLock,Arc,Weak};
pub struct Member{
    pub uid :String,
    pub u_branch_id:String,
    pub branch_id :String,
    pub inviter :String,
    ///层级
    pub invite_index : RwLock<u32>,
    ///上级用户
    pub inviter_prev : RwLock<Weak<Member>>,
    ///直推用户
    pub invite_departments : RwLock<Vec<Arc<Member>>>,
}

pub(crate) type RefMember = Arc<Member>;

impl Member{
   pub fn new(uid:&str,u_branch_id:&str, branch_id :&str ,inviter:&str,)->Member{
        Member{
            uid:String::from(uid),
            u_branch_id:String::from(u_branch_id),
            branch_id :String::from(branch_id),
            inviter_prev:RwLock::new(Weak::new()),
            invite_departments: RwLock::new(vec![]),
            invite_index:RwLock::new(0),
            inviter:String::from(inviter)
        }
    }
}
///判断相同
impl PartialEq  for Member{
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid && self.u_branch_id == other.u_branch_id
    }
}
trait RefMemberInit {
      ///设置邀请人
   fn set_inviter(&self, inviter : &Arc<Member>);
   ///我的邀请人
   fn my_inviter(&self) -> Option<Arc<Member>>;
   ///添加直推
   fn set_invite_department(&self, member:&Arc<Member>);
}
impl RefMemberInit for RefMember{
    ///设置邀请人
    fn set_inviter(&self, inviter : &Arc<Member>){
        *self.inviter_prev.write().unwrap() = Arc::downgrade(inviter);
        let old = *inviter.invite_index.read().unwrap();
        *self.invite_index.write().unwrap() = old + 1;
    }
    ///我的邀请人
    fn my_inviter(&self) -> Option<Arc<Member>>{
        match self.inviter_prev.try_read() {
            Ok(x)=> x.upgrade(),
            Err(_) => None,
           }
    }
    ///添加直推
    fn set_invite_department(&self,  member:&Arc<Member>){
        self.invite_departments.write().unwrap().push(Arc::clone(member));
    }
 }
pub trait MemberUpdate{
  
   ///添加直推部门
    fn and_invite_child(&self, member:&Arc<Member>);
}

impl MemberUpdate for RefMember{
   
   ///添加直推部门
   fn and_invite_child(&self, member:&Arc<Member>){
       member.set_inviter(self);
       self.set_invite_department(member);
   }
}


///测试打印
pub fn print_node(node : &RefMember){
    for c in node.invite_departments.read().unwrap().iter(){
        println!("父：{}  子 ：{}", node.uid, c.uid);
        print_node(&c.clone());
    }
}

