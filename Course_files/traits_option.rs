fn main() {
  // Trait = rasgo

  let edad: Option<i32> = Some(20);
  
  if edad.es_mayor_de_edad() {
    // hago algo 
  } else {
    // otra cosa
  }
}

trait LicenciaConducir {
  fn es_mayor_de_edad(&self) -> bool;
}

impl LicenciaConducir for Option<i32> {
  fn es_mayor_de_edad(&self) -> bool {
    match self {
      Some(edad) => edad > &18,
      None => false,
    }
  }
}
