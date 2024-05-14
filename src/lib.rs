use near_sdk::collections::UnorderedMap; //5.1.0 ver
use near_sdk::{log, near, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize,BorshSerialize};

#[near_bindgen] //니어 초기화 매크로?
#[derive(BorshDeserialize, BorshSerialize)] //직렬화
pub struct NearPowerGame {
    user_lists: UnorderedMap<String,UserGameData>
}

impl Default for NearPowerGame {
    fn default() -> Self {
        Self {
            user_lists : UnorderedMap::new(b"user_lists".to_vec()),
        }
    }
}

#[near_bindgen]
impl NearPowerGame {

}

#[derive( BorshSerialize,BorshDeserialize)]
pub struct UserGameData {
    id : String,
    near_power : i32,
    near_money : i32,
    owner : String,
}