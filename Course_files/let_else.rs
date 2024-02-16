fn main() {
    // let-else
    let algun_numero: Option<i32> = None;

    // forma 1
    match algun_numero {
        Some(numero) => {
            println!("número válido: {}", numero)
            //muchas cosas más con número
        }
        None => println!("número inválido"),
    }

    // forma 2
    if let Some(numero) = algun_numero {
        println!("número válido: {}", numero);
        //muchas cosas más con número
    } else {
        println!("número inválido");
    }

    //rust 1.65
    //let-else
    let Some(numero) = algun_numero else {
        //va a caer si no puede hacer match
        panic!("El número no es válido");
    };

    println!("número válido: {}", numero);

    if let Ok(resultado) = muchos_calculos(algun_numero) {
        println!("resultado: {}", resultado);
    }
}

fn muchos_calculos(input: Option<i32>) -> Result<i32, &'static str> {
    let Some(mut numero) = input else {
        return Err("número no válido");
    };
    //muchas operaciones
    numero = numero + 1000;
    //...

    return Ok(numero);
}
