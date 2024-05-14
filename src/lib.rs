use near_sdk::collections::UnorderedMap; //5.1.0 ver
use near_sdk::{AccountId, env, log, near, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize,BorshSerialize};

#[near_bindgen] //니어 초기화 매크로?
#[derive(BorshDeserialize, BorshSerialize)] //직렬화
pub struct NearPowerGame {
    user_lists: UnorderedMap<String,UserGameData>,
    user_count: i16
}

impl Default for NearPowerGame {
    fn default() -> Self {
        Self {
            user_lists : UnorderedMap::new(b"user_lists".to_vec()),
            user_count : 0
        }
    }
}

#[near_bindgen]
impl NearPowerGame {

}

#[derive( BorshSerialize,BorshDeserialize)]
pub struct UserGameData {
    id : i16,
    near_power : i32,
    near_money : i32,
    owner : AccountId, //signer의 아이디를 받는다.
}

impl UserGameData {
    pub fn new_user(&self,user_count : i16,
                    user_base_money : i32,
                    user_base_power : i32)->Self {
        Self {
            id : user_count,
            near_power : user_base_power,
            near_money : user_base_money,
            owner : env::signer_account_id()
        }
    }
}