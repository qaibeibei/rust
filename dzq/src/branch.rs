use mysql::*;
use mysql::prelude::*;
///分支结构
#[derive(Debug)]
pub struct Branch{
    //用户Id
    pub uid :String,
    //用户分支Id 
    pub u_branch_id:String,
    //分支Id
    pub branch_id :String,
}


//分支相关
pub trait BranchDB{
    fn init(&mut self);
    fn get_user_branches(&mut self )-> Vec<Branch>;
    fn init_tb(&mut self);
    fn init_branch_index_tb(&mut self, branch: &str);
    fn init_branch_pre_index_tb( &mut self, branch_id:&str, sort:&str, index :&str);
    fn init_index_tb(&mut self, branches:Vec<String>);
}
impl BranchDB for PooledConn {
   
    fn init(&mut self){
        
        //创建系统分支
        self.exec_drop( r"INSERT INTO userbranch (uid, u_branch_id, branch_id)
        SELECT * FROM (SELECT  'sysuserid','root','system') AS tmp
        WHERE NOT EXISTS (
            SELECT u_branch_id FROM userbranch WHERE u_branch_id = 'root'
        ) LIMIT 1;",()).unwrap();

        let branches: Vec<String> = self.query_map("SELECT branch_id FROM `userbranch` GROUP BY `branch_id`"
        ,|branch_id|{
            branch_id
        }).unwrap();
        self.init_index_tb(branches);
     }
    //获取分支root用户
 fn get_user_branches(&mut self )-> Vec<Branch>{
    let branches: Vec<Branch> = self.query_map("SELECT u_branch_id,branch_id,uid FROM `userbranch`;"
        ,|(u_branch_id,branch_id,uid)|{
            Branch{u_branch_id,branch_id,uid}
        }).unwrap();
    branches
}
     //初始化分支表
 fn init_tb(&mut self){
        self.query_drop("CREATE TABLE IF NOT Exists `userbranch` (
            `u_branch_id` varchar(64) CHARACTER SET utf8mb4 NOT NULL,
            `branch_id` varchar(64) CHARACTER SET utf8mb4 NOT NULL,
            `uid` varchar(64) CHARACTER SET utf8mb4 NOT NULL,
            PRIMARY KEY (`u_branch_id`)
          ) ENGINE=InnoDB DEFAULT CHARSET=latin1").unwrap();
        }
    //关系总表初始化
fn init_branch_index_tb(&mut self, branch: &str){
    self.query_drop(format!("CREATE TABLE IF NOT Exists  `{}_ship` (
        `uid` varchar(64) CHARACTER SET utf8mb4 NOT NULL,
        `u_branch_id` varchar(64) CHARACTER SET utf8mb4 NOT NULL,
        `inviter` varchar(64) CHARACTER SET utf8mb4 NOT NULL,
        `setter` varchar(64) CHARACTER SET utf8mb4 NOT NULL DEFAULT '',
        `setup_time` datetime(6) NOT NULL,
        PRIMARY KEY (`uid`,`u_branch_id`)
      ) ENGINE=InnoDB DEFAULT CHARSET=latin1;",branch)).unwrap();
}
//关系闭包表初始化
fn init_branch_pre_index_tb( &mut self, branch_id:&str, sort:&str, index :&str){
    self.query_drop(format!("CREATE TABLE IF NOT Exists  `{}_{}_pre_{}` (
        `uid` varchar(64) CHARACTER SET utf8mb4 NOT NULL,
        `u_branch_id` varchar(64) CHARACTER SET utf8mb4 NOT NULL,
        `pre_uid` varchar(64) CHARACTER SET utf8mb4 NOT NULL,
        `index` int(11) NOT NULL,
        PRIMARY KEY (`uid`,`u_branch_id`,`pre_uid`)
      ) ENGINE=InnoDB DEFAULT CHARSET=latin1;",branch_id,sort,index)).unwrap();
}
//初始化
 fn init_index_tb(&mut self, branches:Vec<String>){
    for item in branches{
        self.init_branch_index_tb(&item);//总表
        for i in 0..0x10{//闭包表
            self.init_branch_pre_index_tb(&item,"invite",&format!("{:x}",i));
            self.init_branch_pre_index_tb(&item,"setter",&format!("{:x}",i));
        }
    }
}

}

