use std::env;

mod buscaminas;
mod constantes;
mod file;
mod usr;

/// Función Principal. Redirige todas las funcionalidades del sistema.
/// Da la bienvenida al usuario y le informa la finalizazión del proceso.
/// Lanza el procesamiento de traducción del mapa.
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    usr::entrada();

    usr::procesamiento(path);

    buscaminas::buscaminas(&file::leer(path));

    usr::salida();
}
