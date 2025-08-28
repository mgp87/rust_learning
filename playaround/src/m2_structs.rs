#[cfg(test)]
mod tests {

    #[derive(Debug)]
    #[allow(dead_code)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    impl User {
        fn increment_signin_count(&mut self) {
            self.sign_in_count += 1;
        }

        fn change_email(&mut self, new_email: &str) {
            self.email = String::from(new_email);
        }

        fn change_username(&mut self, new_username: &str) {
            self.username = String::from(new_username);
        }
    }

    #[test]
    fn test_structs() {
        let mut user1 = User {
            username: String::from("user1"),
            email: String::from("user1@example.com"),
            sign_in_count: 1,
            active: true,
        };
        user1.change_username("anotherusername");
        user1.change_email("anotheremail@example.com");
        user1.increment_signin_count();
        dbg!(user1);
    }
}
