use std::io;

fn main() {
    println!("-----Calculadora de porcentaje de faltas------");
    println!("¿Qué asignatura quieres calcular?: iso/gbd/fol/fh/pax/llm ");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let asignatura = input.trim();
            println!("Seleccionaste la asignatura: {}", asignatura);

            let horas = match asignatura {
                "iso" => 210,
                "gbd" => 260,
                "fol" => 90,
                "fh" => 100,
                "pax" => 170,
                "llm" => 130,
                _ => {
                    println!("No hay información de esta asignatura");
                    return;
                }
            };
            println!("Esta asignatura tiene {} horas", horas);
            println!("Cuantas horas has faltado a {}?", asignatura);

            let mut faltas = String::new();
            match io::stdin().read_line(&mut faltas) {
                Ok(_) => {
                    let faltas: i32 = match faltas.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Horas no válidas");
                            return;
                        }
                    };
                    println!("Has faltado {} horas a esa asignatura? xd erga mano", faltas);
                }
                Err(error) => {
                    println!("Error al leer la entrada de faltas: {}", error);
                }
            }
        }
        Err(error) => {
            println!("Error al leer la entrada: {}", error);
        }
    }
}
