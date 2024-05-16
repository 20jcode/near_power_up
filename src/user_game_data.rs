use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env};

#[derive( BorshSerialize,BorshDeserialize)]
pub struct UserGameData {
    near_power : i32,
    near_money : i32,
    owner : AccountId, //signer의 아이디를 받는다.
}

impl UserGameData {
    pub fn new_user(
                    user_base_money : i32,
                    user_base_power : i32)->Self {
        Self {
            near_power : user_base_power,
            near_money : user_base_money,
            owner : env::signer_account_id()
        }
    }

    pub fn get_near_power(&self) -> i32 {
        return self.near_power
    }

    pub fn get_near_money(&self) -> i32 {
        return self.near_money
    }
}