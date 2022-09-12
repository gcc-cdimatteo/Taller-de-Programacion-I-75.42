use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::constantes;

/// Lee el archivo del path recibido y lo convierte a string.
/// Recibe el path donde se almacene el archivo a leer.
/// Entrega la lectura en formato string del archivo.
pub fn leer(path: &str) -> String {
    fs::read_to_string(path).expect("[ERROR] Ingrese una ruta válida")
}

/// Guarda el tablero en formato de vector de vectores de u8 en el path definido en el móudlo de constantes llamado _PATH_SALIDA. Convierte a string cada uno de los elementos del tablero según las constantes definidas en el módulo de constantes.
/// Recibe el tablero en formato de vector de vectores de u8.
pub fn guardar(tablero: &Vec<Vec<u8>>) {
    let mut file = File::create(constantes::_PATH_SALIDA).expect("fallo");
    for i in tablero {
        for j in i {
            let mut s;
            if *j == constantes::_VACIO_U8 {
                s = String::from(constantes::_VACIO_STR);
            } else if *j == constantes::_BOMBA_U8 {
                s = String::from(constantes::_BOMBA_STR);
            } else {
                s = String::from(&*j.to_string());
            }
            s.push(' ');
            file.write_all(s.as_bytes()).expect("fallo");
        }
        file.write_all(String::from(constantes::_ENTER_STR).as_bytes())
            .expect("fallo");
    }
}
