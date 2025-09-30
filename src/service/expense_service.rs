use crate::model::Expense;
use crate::persistence::ExpenseRepository;

pub struct ExpenseService<'a> {
    repo: ExpenseRepository<'a>,
}

impl<'a> ExpenseService<'a> {
    pub fn new(repo: ExpenseRepository<'a>) -> Self {
        ExpenseService { repo }
    }

    pub async fn create_expense(
        &self,
        expense: &Expense,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if expense.amount() <= 0.0 {
            return Err("Expense amount must be greater than 0".into());
        }

        self.repo.save_expense(expense).await?;
        Ok(())
    }
}
