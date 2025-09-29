pub struct Expense {
    #[getset(get = "pub", set = "pub")]
    description: String,
    #[getset(get = "pub", set = "pub")]
    amount: f64,
    #[getset(get = "pub", set = "pub")]
    date: String,
    #[getset(get = "pub", set = "pub")]
    user: User
}

impl Expense {
    fn new(description: &str, amount: f64, date: &str, user: User) -> Self {
        Expense {
            description: description.to_string(),
            amount,
            date: date.to_string(),
            user: user,
        }
    }
}