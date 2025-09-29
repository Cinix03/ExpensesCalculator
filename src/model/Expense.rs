use serde::{Deserialize, Serialize};
use getset::{Getters, Setters};
use crate::model::user::User;
#[derive(Serialize, Deserialize, Debug, Getters, Setters)]
pub struct Expense {
    #[getset(get = "pub", set = "pub")]
    description: String,
    #[getset(get = "pub", set = "pub")]
    amount: f64,
    #[getset(get = "pub", set = "pub")]
    date: String,
    #[getset(get = "pub", set = "pub")]
    user: User,
}

impl Expense {
    pub fn new(description: &str, amount: f64, date: &str, user: User) -> Self {
        Expense {
            description: description.to_string(),
            amount,
            date: date.to_string(),
            user,
        }
    }
}