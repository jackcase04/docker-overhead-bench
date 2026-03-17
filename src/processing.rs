use std::collections::HashMap;

use crate::transaction::Transaction;

pub struct Processor {
    pub users: HashMap<u32, User>
}

pub struct User {
    pub f_name: String,
    pub l_name: String,
    pub home_lat: f64,
    pub home_long: f64
}

impl Processor {
    pub fn process_transaction(&self, transaction: Transaction) -> bool {
        let user: Option<&User> = self.users.get(&transaction.user_id); 
        
        match user {
            Some(_user) => true,
            None => false
        }
    }
}
