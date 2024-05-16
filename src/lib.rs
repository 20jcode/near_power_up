use near_sdk::collections::UnorderedMap; //5.1.0 ver
use near_sdk::{AccountId, env, log, near, near_bindgen, NearToken, Promise};
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
    //유저 정보 조회
    pub fn get_user(&self,user_id_str : &String) -> Option<UserGameData> {
        return self.user_lists.get(user_id_str);
    }

    #[payable]
    pub fn power_up(&mut self){
        //signer의 id를 통해 해당 user game data를 찾는다.
        match self.user_lists.get(&env::signer_account_id().to_string()){
            Some(ref mut user_game_data) => { //일치하는 유저를 찾으면

                let power_up_price : i32 = 10; //임시값
                let price_token = NearToken::from_yoctonear(power_up_price as u128); //10^-24 니어 토큰만큼을 price_token으로 변환

                let deposit = env::attached_deposit(); //제출한 금액이 일치하는 지 확인하기 위해, 사용자의 deposit 확인?


                assert_eq!(deposit,price_token); //확인

                let master = env::current_account_id(); //스마트 컨트렉트 계정(컨트렉트 배포자?)의 id

                Promise::new(master.parse().unwrap()).transfer(price_token); //토큰 전송

                user_game_data.power_up(); //user 파워 업
            },
            _ => { //match 된 유저가 없다면
                env::panic_str("no found user");
            }
        }
    }


}



#[cfg(test)]
mod tests {
    use super::*;
    #[test] //user 조회 기능
    fn create_user_test(){
        let mut contract = NearPowerGame::default();
        contract.create_user();
        let id = env::signer_account_id().to_string();

        assert_eq!(id,contract.get_user(&id).unwrap().get_user_id());
    }


}
