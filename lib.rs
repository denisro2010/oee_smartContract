
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Incrementer {
        value: i32,
        valid: bool,
        my_value: ink_storage::Mapping<AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = init_value;
                let caller = Self::env().caller();
                contract.my_value.insert(&caller, &0);
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            // Even though we're not explicitly initializing the `Mapping`,
            // we still need to call this
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = Default::default();
            })
        }

        #[ink(message)]
        pub fn get_oee(&self) -> i32 {
            self.value
        }
        
        #[ink(message)]
        pub fn is_valid(&self) -> bool {
            self.valid
        }

        #[ink(message)]
        pub fn set_oee(&mut self, by: i32) {
            self.value = by;
        }
        
        #[ink(message)]
        pub fn validate_oee(&mut self){
            let val = self.get_oee();
            if val > 59 && val < 101{
            	self.valid = true;
            }else{
                self.valid = false;
            }
        }     

    }

}
