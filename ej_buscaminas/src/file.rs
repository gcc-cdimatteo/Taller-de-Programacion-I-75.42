use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[path = "../src/constantes.rs"]
mod constantes;

pub fn _leer(path: &str) -> String {
    fs::read_to_string(path).expect("[ERROR] Ingrese una ruta válida")
}

pub fn _guardar(tablero: &Vec<Vec<u8>>) {
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
            s.push_str(" ");
            file.write_all(s.as_bytes()).expect("fallo");
        }
        file.write_all(String::from(constantes::_ENTER_STR).as_bytes())
            .expect("fallo");
    }
}
