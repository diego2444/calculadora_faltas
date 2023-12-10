use std::io;

fn main() {
    println!("-----Calculadora de porcentaje de faltas------");
    println!("¿Qué asignatura quieres calcular?: iso/gbd/fol/fh/pax/llm ");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let asignatura = input.trim();
            println!("Has seleccionado: {}", asignatura);

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
            println!("Esta asignatura tiene {} horas.", horas);
            println!("¿Cuántas horas has faltado a {}?", asignatura);

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
                    let porcentaje_faltas = (faltas as f64 / horas as f64) * 100.0;
                    println!("Tu porcentaje de faltas en {} es: {:.2}%", asignatura, porcentaje_faltas);
                    
                    if porcentaje_faltas > 15.0 {
                        println!("Nooo, ya has superado el 15% :(")
                    }
                    else if porcentaje_faltas >= 10.0 {
                        println!("¡Ojo! has faltado más del 10% en {}.", asignatura)
                    } else {
                        println!("Por ahora no tendrías que tener problemas, sigue así :)")
                    }
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
