use std::fmt;

#[derive(Debug)]
struct User {
  nombre: String,
  edad: i32,
}

impl fmt::Display for User {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
    writeln!(f, "{} ({})", self.nombre, self.edad)
  }
}

fn main() {
  let usuario = User {
    nombre: "Julio Andres".to_string(),
    edad: 20,
  };

  println!("{}", usuario);
  println!("{:?}", usuario);
}
