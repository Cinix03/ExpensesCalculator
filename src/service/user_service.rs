use crate::model::User;
use crate::persistence::UserRepository;

pub struct UserService<'a> {
    firebase: Firebase<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(repo: UserRepository<'a>) -> Self {
        UserService { repo }
    }

    pub async fn create_user(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {

        self.repo.save_user(user).await?;
        Ok(())
    }
}
