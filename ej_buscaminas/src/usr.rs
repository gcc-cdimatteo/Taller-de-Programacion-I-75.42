use core::time;
use std::thread::sleep;

use crate::constantes;

/// Bienvenida al usuario.
pub(crate) fn entrada() {
    println!("Bienvenido al Buscaminas :)");
}

/// Evalúa la validez del path enviado por el usuario.
pub(crate) fn procesamiento(path: &str) {
    println!("Vamos a procesar el mapa en la ruta {}", path);
    println!("Por favor, aguarde mientras trabajamos...");
    sleep(time::Duration::from_secs(1));
}

/// Saluda al usuario y le informa dónde se almacena la salida procesada.
pub(crate) fn salida() {
    println!("Todo listo!");
    println!(
        "Puede revisar el mapa revelado en la ruta [/{}]",
        constantes::_PATH_SALIDA
    );
    println!("Nos vemos! :)");
}
