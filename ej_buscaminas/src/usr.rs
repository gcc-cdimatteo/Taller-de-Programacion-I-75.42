use core::time;
use std::thread::sleep;

#[path = "../src/constantes.rs"]
mod constantes;

pub(crate) fn entrada() {
    println!("Bienvenido al Buscaminas :)");
}

pub(crate) fn procesamiento(path: &str) {
    println!("Vamos a procesar el mapa en la ruta {}", path);
    println!("Por favor, aguarde mientras trabajamos...");
    sleep(time::Duration::from_secs(1));
}
pub(crate) fn salida() {
    println!("Todo listo!");
    println!("Puede revisar el mapa revelado en la ruta [/{}]", constantes::_PATH_SALIDA);
    println!("Nos vemos! :)");
}

