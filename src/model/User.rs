pub struct User {
    #[getset(get = "pub", set = "pub")]
    name: String,
    #[getset(get = "pub", set = "pub")]
    username: String,
    #[getset(get = "pub", set = "pub")]
    password: String,
}

impl User {
    fn new(name: &str, username: &str, password: &str) -> Self {
        User {
            name: name.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}