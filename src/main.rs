// use member_server::member;
// use member_server::new_member;
// use async_std::task::{sleep, spawn};
use dzq;
use member_server::new_member2;
use simple_poolweb::web::web_init;
// use std::time::Duration;

// use std::time::Instant;

use std::time::{Duration, SystemTime};

use rand::prelude::*;
use hashbrown::HashMap;
use std::collections::HashMap;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Aa {
    uid: u64,
    balance: u8,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct  StakingDetail {
    /// 质押币
    staking_coin: Vec<u8>,
    /// 金额
    amount: u128,
    /// 质押类型
    bill_type: StakingBillType,
    /// index
    index: u32,
    /// 时间
    ts: u64,
    /// 区块
    block_number: u32,
}

pub enum StakingBillType {
    /// 质押
    StakingIn,
    /// 解压
    StakingOut,
}

pub struct UserData {
    /// 地址
    address: AccountId,
    /// 质押金额
    staking_amount: u128,
    /// 上次结算 index
    last_reward_index: u32,
    /// 上次结算区块
    last_reward_block_number: u32,
    /// 上次质押数量
    last_staking_amount: u128,
}


async fn deposit(u: u32, amount: u128, block_number: u32) {


}
async fn bounds_amount(block_number: u32) {
    let user_data = UserData {
        address: (),
        staking_amount: 0,
        last_reward_index: 0,
        last_reward_block_number: 0,
        last_staking_amount: 0
    };

    let block_reward = 10u128;
    // 区块 -> (总量, block_number)
    let mut block_total_amount: Vec<(u128, u32)> = vec![];
    // 区块 -> index
    let mut block_index: HashMap<u32, u32> = HashMap::new();
    let mut staking_bill1: HashMap<u32, Vec<StakingDetail>> = HashMap::new();
    let mut index = 0;

    let mut start_index = 0;
    let mut start_block_number = user_data.last_reward_block_number;
    let mut start_total_amount = user_data.last_staking_amount;


    let mut bills: Vec<StakingDetail> = vec![];
    {
        bills.push(StakingDetail {
            staking_coin: vec![],
            amount: 100,
            bill_type: StakingBillType::StakingIn,
            index: 0,
            ts: 0,
            block_number: 2
        });
        block_total_amount.push((100, 2));
        block_index.insert(2, 0);
        bills.push(StakingDetail {
            staking_coin: vec![],
            amount: 100,
            bill_type: StakingBillType::StakingIn,
            index: 1,
            ts: 0,
            block_number: 4
        });
        block_total_amount.push((200, 4));
        block_index.insert(4, 1);
        bills.push(StakingDetail {
            staking_coin: vec![],
            amount: 50,
            bill_type: StakingBillType::StakingOut,
            index: 2,
            ts: 0,
            block_number: 6
        });
        block_total_amount.push((250, 6));
        block_index.insert(6, 2);
        index = 2;
    }

    bills.sort_by(|a, b| a.index.cmp(&b.index));
    if user_data.last_reward_block_number == 0 { //没领过
        start_index = bills[0].index;
        if Some(x) = block_total_amount.get(start_index) {
            start_block_number = x;
        }
    }

    let mut is_first = true;
    let mut total_bounds = 0u128;
    println!(" {:?}", index);

    for i in start_index..index {
        println!("for {:?}", i);
        if let Some(total_amount) = block_total_amount.get(index) {
            if x.1 == block_number {  // 查询的块不在结算范围内
                continue;
            }
            if user_data.last_reward_block_number == 0 || !is_first {  // 没领取过，首次进来要统计质押数量 或者  不是第一次进来（避免在两个index 的情况）
                let index_bills: Vec<&StakingDetail> = bills.iter().filter(|a| a.index == i).collect();
                if index_bills.len() > 0 {
                    for index_bill in index_bills { // 这个地方要排序
                        match index_bill.bill_type {
                            StakingBillType::StakingIn => {
                                start_total_amount += index_bill.amount;
                            }
                            StakingBillType::StakingOut => {
                                start_total_amount -= index_bill.amount;
                            }
                        }
                    }
                }
            }

            if let Some(x) = block_total_amount.get(index + 1) {
                total_bounds += (x.1 - start_block_number) * start_total_amount * block_reward / total_amount.0;
                println!("total_bounds  {:?}", total_bounds);
            } else {
                total_bounds += (block_number - start_block_number) * start_total_amount * block_reward / total_amount.0;
                println!("total_bounds  {:?}", total_bounds);
            }
        }
        is_first = false;
    }


    println!("bounds  {:?}", total_bounds);
}

#[async_std::main]
async fn main() {
    let mut stack = Vec::new();
    let mut rng = rand::thread_rng();

     bounds_amount(8).await;
    // Each thread has an automatically-initialised random number generator:

    // Integers are uniformly distributed over the type's whole range:

    {
        // let n1: u8 = rng.gen();
        // let n2: u16 = rng.gen();
        // println!("n1 is :{}", n1);
        // println!("n2 is :{}", n2);
        // println!("here is a random u32 number:{}", rng.gen::<u32>());
        // println!("here is a random i32 number:{}", rng.gen::<i32>());

        // // Floating point numbers are uniformly distributed in the half-open range [0, 1)
        // println!("Random float: {}", rng.gen::<f64>());

        // //Generates a random value within half-open [0, 10) range (not including 10) with Rng::gen_range.
        // println!("Integer: {}", rng.gen_range(0, 10));
        // println!("Float: {}", rng.gen_range(0.0, 10.0));
    }

    let now = SystemTime::now();
    for _ in 1..10 {
        stack.push(Aa {
            uid: rng.gen::<u64>(),
            balance: rng.gen::<u8>(),
        });
    }

    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("加载 -> {} millis", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    let now1 = SystemTime::now();
    //    let mut fd= stack.iter().filter(|a| a.balance>100).collect().to_vec();

    stack.sort_by(|a, b| a.balance.cmp(&b.balance));

    match now1.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("排序 -> {} millis", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    println!("{}", stack.len());
    println!("{:#?}", stack);
    stack.iter().filter(|a| a.balance > 0).next();
    println!("......................................................");
    let mut top_contries = stack
        .into_iter()
        .filter(|a| a.balance > 0)
        .collect::<Vec<Aa>>();
    top_contries.sort_by(|a, b| a.balance.cmp(&b.balance));
    println!("{}", top_contries.len());
    println!("{:#?}", top_contries);

    let mut asdfdf: Vec<Tem> = Vec::new();

    let mut level = 0u8;
    let mut asd = 0u8;
    for aaa in top_contries {
        if asd != aaa.balance {
            asd = aaa.balance;
            level = level + 1;
        }
        let mut uuu = asdfdf.iter_mut().filter(|a| a.uid == aaa.uid).next();

        match uuu {
            None => {
                asdfdf.push(Tem {
                    uid: aaa.uid,
                    balance: aaa.balance,
                    index: level,
                });
            }
            Some(x) => {
                x.balance = aaa.balance;
            }
        }
    }
    // println!("Hello, world1! {:#?}", asdfdf);

    let aaa1 = asdfdf
        .iter()
        .map(|a| a.balance as u32)
        .collect::<Vec<u32>>();
    println!("Hello, world2! {:#?}", aaa1);
    for elem in 1..10 {
        let sds: u32 = aaa1.iter().sum();
        let sds = aaa1.iter().max();
    }

    let mut last_max  = 0u8;

    // 等级， 是否已经平级过，
    let mut last_level : Vec<(u8, bool, u128, u128)> = Vec::new();
    last_level.push((1, false, 0, 0));
    last_level.push((2, false, 0, 0));
    last_level.push((3, false, 0, 0));
    last_level.push((4, false, 0, 0));
    last_level.push((5, false, 0, 0));

    println!("{:#?}", last_level);

    // println!("Hello, world3! {:#?}", sds);
    // let now = Instant::now();
    // println!("{:#?}",now);

    // member::init();
    // new_member2::init();
    // dzq::init();
    // for i in 1..10000 {
    //     println!("Hello, world! {}", i);
    // }
    // web_init();
    println!("Hello, world3! {:#?}", u32::max_value());
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tem {
    uid: u64,
    balance: u8,
    index: u8,
}
