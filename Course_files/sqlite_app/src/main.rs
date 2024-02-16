use rusqlite::{Connection, Result};

fn main() {
    
  let conn = create_database();
  create_table(&conn);
  insert_user(&conn, "Julio Andres");
  
  let user = get_user(&conn, 1);
  println!("user: {:?}", user.unwrap());
}

fn get_user(conn: &Connection, user_id: i32) -> Result<String> {
  let mut stm = conn.prepare("SELECT name from users WHERE id = ?1")?; // statement
  let users = stm.query_map([user_id], |row| {
    let name: String = row.get(0)?;
    Ok(name)
  })?;

  for user in users {
    return Ok(user.unwrap());
  }

  Ok("No se encontró el usuario".to_string())
}

fn insert_user(conn: &Connection, user: &str) {
  let result = conn.execute("INSERT INTO users (name) values (?1)", &[user]);
  match result {
    Ok(_) => println!("Usuario insertado con éxito"),
    Err(e) => panic!("No se pudo insertar usuario, error: {}", e)
  }
}

fn create_table(conn: &Connection) {
  let result = conn.execute("
  CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
  )", []);
  match result {
    Ok(_) => println!("Tabla creada con éxito"),
    Err(e) => panic!("No se pudo crear la tabla, error: {}", e)
  }
}

fn create_database() -> Connection {
  let result = Connection::open("users.db");
  match result {
    Ok(_) => {
      println!("Base de datos abierta con éxito");
      return result.unwrap();
  },
    Err(e) => panic!("Base de datos no se pudo abrir, error: {}", e)
  }
}
