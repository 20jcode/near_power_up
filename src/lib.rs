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
    user_base_power: i32,
    user_base_money : i32,
}

impl Default for NearPowerGame {
    fn default() -> Self { //초기 init
        Self {
            user_lists : UnorderedMap::new(b"user_lists".to_vec()),
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

        let user_game_data = UserGameData::new_user(
            self.user_base_money,
            self.user_base_power
        );
        let user_id_str = env::signer_account_id().to_string();
        self.user_lists.insert(&user_id_str,&user_game_data);
    }

    pub fn get_user(&self,user_id_str : &String) -> Option<UserGameData> {
        return self.user_lists.get(user_id_str);
    }


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_test(){
        let mut contract = NearPowerGame::default();
        contract.create_user();
        let id = env::signer_account_id().to_string();

        assert_eq!(id,contract.get_user(&id).unwrap().get_user_id());

    }
}
