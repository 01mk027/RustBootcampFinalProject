pub struct SecurityManager {
    pub current_user: Option<User>,
}

pub struct User {
    pub username: String,
    pub role: UserRole,
}

#[derive(PartialEq)]
pub enum UserRole {
    Admin,
    Manager,
    Employee,
}
#[allow(dead_code, unused_variables)]
impl SecurityManager {
    pub fn new() -> Self {
        SecurityManager { current_user: None }
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<&User, &'static str> {
        // Here you'd have logic to verify the username and password, such as checking a database.
        // For this example, we'll just hardcode a user.

        if username == "admin" && password == "password" {
            self.current_user = Some(User {
                username: username.to_string(),
                role: UserRole::Admin,
            });
            let user = match &self.current_user {
                Some(user) => user,
                None => panic!(""),
            };
            Ok(user)
        } else {
            Err("Invalid username or password")
        }
    }

    pub fn logout(&mut self) {
        self.current_user = None;
    }

    pub fn check_permission(&self, required_role: UserRole) -> bool {
        if let Some(ref user) = self.current_user {
            match user.role {
                UserRole::Admin => true,
                _ if required_role == UserRole::Manager && user.role == UserRole::Manager => true,
                _ if required_role == UserRole::Employee => true,
                _ => false,
            }
        } else {
            false
        }
    }
}
