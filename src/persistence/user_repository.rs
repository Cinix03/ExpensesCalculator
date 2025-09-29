use firebase_rs::Firebase;
use crate::model::User;

pub struct UserRepository<'a> {
    firebase: &'a Firebase,
}

impl<'a> UserRepository<'a> {
    pub fn new(firebase: &'a Firebase) -> Self {
        UserRepository { firebase }
    }

    pub async fn save_user(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("users/{}", user.username());
        self.firebase.at(&path).set(user).await?;
        Ok(())
    }
}
