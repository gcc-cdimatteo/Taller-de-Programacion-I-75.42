//! # Buscaminas
//! ###### Carolina Di Matteo - cdimatteo@fi.uba.ar - 103963
//! ______________
//! ##### Introducción
//! La presente entrega contiene las funcionalidades pedidas para
//! el trabajo practico nº1 de la materia Taller de Programacion I - curso Deymonnaz.
//!
//! ##### Objetivo
//! El objetivo de este trabajo practico consta de simular la logica del juego
//! [Buscaminas](https://es.wikipedia.org/wiki/Buscaminas)
//! implementandolo en
//! [Rust](https://doc.rust-lang.org/rust-by-example/index.html)
//! siguiendo los conceptos trabajados en clase.
//!
//! Para acceder al repositorio donde fue desarrollado el mismo
//! se puede visitar el siguiente [enlace](https://github.com/valencorrea/Buscaminas).
//!
//! ##### Ejecución
//! Para comenzar a utilizar el programa se deberá hacer uso del comando *cargo run* seguido
//! de la ruta en donde se encuentra el archivo de entrada.
//! En particular, los archivos de entrada estan dentro de */main/src/mapas*, ruta en
//! donde tambien se guardara el archivo de salida.
//!
//! *Ejemplo: cargo run mapas/mapa_input.txt*
//!
//! Otros comandos de interes:
//! - *cargo test*
//! - *cargo fmt*
//! - *cargo clippy*
//! - *cargo doc --open*

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
