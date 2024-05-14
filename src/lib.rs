use near_sdk::collections::UnorderedMap; //5.1.0 ver
use near_sdk::{AccountId, env, log, near, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize,BorshSerialize};
use near_sdk::env::log_str;

mod user_game_data;
use user_game_data::UserGameData;


#[near_bindgen] //니어 초기화 매크로?
#[derive(BorshDeserialize, BorshSerialize)] //직렬화
pub struct NearPowerGame {
    user_lists: UnorderedMap<String, UserGameData>,
    user_count: i16,
    user_base_power: i32,
    user_base_money : i32,
}

impl Default for NearPowerGame {
    fn default() -> Self { //초기 init
        Self {
            user_lists : UnorderedMap::new(b"user_lists".to_vec()),
            user_count : 0,
            user_base_money : 5,
            user_base_power : 0,
        }
    }
}

#[near_bindgen]
impl NearPowerGame {
    pub fn set_base_money(&mut self,change_value : i32) {
        //todo : contract owner verify
        self.user_base_money = change_value;
        log_str("change user base money");

    }
    pub fn set_base_power(&mut self,change_value : i32){
        //todo : contract owner verify
        self.user_base_power = change_value;
        log_str("change user base power");
    }

    //신규유저 생성
    pub fn create_user(&mut self){
        let user_value = self.user_count +1;
        self.user_count = user_value;
        //좀 더 더하기를 편하게 하는 방법은 없는지?

        let user_game_data = UserGameData::new_user(
            self.user_count,
            self.user_base_money,
            self.user_base_power
        );
        self.user_lists.insert(&user_game_data.id.to_string(),&user_game_data);
    }

    pub fn get_user(&self,user_id_str : &String) -> String {
        return match self.user_lists.get(user_id_str) {
            Some(ref ugd ) => {
                ugd.id.to_string()
            }

            _ => {
                "0".parse().unwrap()
            }
        }
    }


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_test(){
        let mut contract = NearPowerGame::default();
        let user_id = "1";
        contract.create_user();
        assert_eq!(user_id,contract.get_user(&user_id.to_string()));
    }
}
