#![allow(unreachable_code, unused_variables)]
fn main() {
    let algun_numero: Option<i32> = None;

    let result: i32 = 'calcular_porcentaje: {
        'muchos_calculos: loop {
            let Some(num) = algun_numero else {
                break 'calcular_porcentaje 0;
            };

            if num > 100 {
                break 'muchos_calculos 100;
            } else {
                break 'calcular_porcentaje num;
            }
        }
    };

    println!("el resultado es: {}", result);
}
