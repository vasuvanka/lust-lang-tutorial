# Split code into modules

Besides helping you to better organize your code, modules also provide privacy guarantees to your values, types, and methods.

Take a look at this example, in which we model a simplified authentication API:

```rs
mod authentication {
    pub struct User {
        username: String,
        password_hash: u64,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
            }
        }    
    }
    fn hash_password(input: &str) -> u64 { /*...*/ }
}

fn main() {

    let user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.username);
    println!("The password is: {}", user.password_hash);

}
```