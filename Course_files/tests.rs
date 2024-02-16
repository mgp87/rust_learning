// Tests -> unit tests = tests unitarios
fn main() {

  // assert = asegurar
  // assert!(expression)
  // assert_eq!(left, right)
  // assert_ne!(left, right)
  // panic!
}

fn dividir(a: i32, b: i32) -> i32 {
  if b == 0 {
    panic!("No puedes dividir por cero!!");
  }
  a/b
}

fn sumar (a: i32, b: i32) -> i32 {
  a + b
}

fn codigo_solo_numeros(codigo: &str) -> bool {
  codigo.chars().all(char::is_numeric)
}

#[test]
#[ignore]
fn check_codigo_con_numeros() {
  let result = codigo_solo_numeros("98302334");
  assert!(result)
}

#[test] 
#[ignore]
fn check_codigo_con_letras() {
  let codigo = "983L02334";
  let result = codigo_solo_numeros(codigo);
  assert!(!result, "El código contiene letras, y lo validó como correcto, Código usado: {}", codigo);
}

#[test]
#[ignore]
fn sumar_bien() {
  assert_eq!(sumar(2, 2), 4);
}

#[test]
#[should_panic(expected = "No puedes dividir por cero!!")] // debe [arrojar] panic
fn test_division_cero() {
  dividir(100, 0);
}
