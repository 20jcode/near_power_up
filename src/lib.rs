use near_sdk::store::unordered_map;
use near_sdk::{log, near, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize,BorshSerialize};

#[near_bindgen] //니어 초기화 매크로?
#[derive(BorshDeserialize, BorshSerialize,Default)] //직렬화
pub struct NearPower {
    near_power : unordered_map<String,i16>,
    my_money : i16,
}

#[near_bindgen]
impl NearPower {
    #[init]
    pub fn new() -> Self {
        Self {
            near_power : 0,
            my_money : 0,
        }
    }
    pub fn power_up(&mut self) -> i16 {
        if self.my_money>0 {
            self.my_money : self.my_money -1;
            self.near_power: self.near_power + 1;
        }
        return self.near_power;
    }
    pub fn mining(&mut self) -> i16 {
        self.my_money : self.my_money + 1;
        return self.my_money;
    }
}