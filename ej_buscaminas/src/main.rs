//! # Buscaminas
//! ###### Carolina Di Matteo - cdimatteo@fi.uba.ar - 103963
//! ______________
//! ##### Introducción
//! 
//! ##### Objetivo
//!
//! ##### Ejecución
//! 

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
