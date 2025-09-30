use firebase_rs::Firebase;
use uuid::Uuid;

use crate::model::Expense;

pub struct ExpenseRepository<'a> {
    firebase: &'a Firebase,
}

impl<'a> ExpenseRepository<'a> {
    pub fn new(firebase: &'a Firebase) -> Self {
        ExpenseRepository { firebase }
    }

    pub async fn save_expense(&self, expense: &Expense) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!(
            "{}_{}_{}",
            expense.user().username(),
            expense.date(),
            Uuid::new_v4()
        );
        let path = format!("expenses/{}/{}", expense.user().username(), key);
        self.firebase.at(&path).set(expense).await?;
        Ok(())
    }
}
