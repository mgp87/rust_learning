struct User {
  name: String,
  email: String,
  activo: bool,
  user_role: UserRole,
  website: Website,
}

// Enums = enumaration
enum UserRole {
  BASIC,
  ADMIN,
}

enum Website {
  URL(String),
  INSTAGRAM(String),
  LINKEDIN(String),
  FACEBOOK(String),
}

fn main() {
  let mut user = User {
    name: "Julio".to_string(),
    email: String::from("algo@algo.com"),
    activo: true,
    website: Website::INSTAGRAM(String::from("@eljulioandres")),
    user_role: UserRole::BASIC,
  };
  
  let access = hasAccess(user.user_role);

}

fn hasAccess(user_role: UserRole) -> bool {
  match user_role {
    UserRole::ADMIN => true,
    UserRole::BASIC => false,
  }
}
