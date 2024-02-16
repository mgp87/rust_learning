fn main() {
  // Closures: función que es definida en linea (inline)
  // | = pipe
  let sum = |nro1:i32, nro2:i32| {
    nro1 + nro2
  };
  println!("{}",sum(4,2)); 

  let mut counter = 1;

  let mut incrementar = move || {
    counter += 1;
  };

  let variable = &counter; // borrowing, pedir prestada
  incrementar();
  
  println!("{}", counter);
}