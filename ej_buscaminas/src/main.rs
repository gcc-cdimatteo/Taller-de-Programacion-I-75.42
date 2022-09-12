//! # Buscaminas
//! ###### Carolina Di Matteo - cdimatteo@fi.uba.ar - 103963
//! ______________
//! ##### Introducción
//! A continuación se desarrolla la resolución para los requerimientos detallados para el ejercicio individual del segundo cuatrimestre de 2022, Taller de Programación I curso Deymonnaz.
//! 
//! ##### Objetivo
//! El objetivo de la actividad consiste en procesar el recuento de minas adyacentes a todas las cuadrículas existentes en un tablero de Buscaminas.
//!
//! ##### Ejecución
//! Para comenzar la ejecución del programa, es necesario enviar como parámetro la ruta donde se encuentre el archivo del mapa. 
//! Ejemplo: cargo run mapa.txt
//! 
//! ##### Locación
//! El código se encuentra disponible en el repositorio [Rust](https://doc.rust-lang.org/rust-by-example/index.html)

use std::env;

mod buscaminas;
mod constantes;
mod file;
mod usr;

use crate::file::ErrorArchivo;

/// Función Principal. Redirige todas las funcionalidades del sistema.
/// Da la bienvenida al usuario y le informa la finalizazión del proceso.
/// Lanza el procesamiento de traducción del mapa.
fn main() -> Result<(), ErrorArchivo> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    usr::entrada();

    usr::procesamiento(path);

    match file::leer(path) {
        Ok(data) => match buscaminas::buscaminas(&data) {
            Ok(tablero) => tablero,
            Err(e) => return Err(e),
        },
        Err(e) => return Err(e),
    };

    usr::salida();

    Ok(())
}
