use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::constantes;

#[derive(Debug)]
pub enum ErrorArchivo {
    Lectura(String),
    Escritura(String),
    Creacion(String),
}

/// Lee el archivo del path recibido y lo convierte a string.
/// Recibe el path donde se almacene el archivo a leer.
/// Entrega la lectura en formato string del archivo.
pub fn leer(path: &str) -> Result<String, ErrorArchivo> {
    match fs::read_to_string(path) {
        Ok(data) => Ok(data),
        Err(_e) => Err(ErrorArchivo::Lectura(String::from(
            "No se puede leer el archivo de entrada.",
        ))),
    }
}

/// Guarda el tablero en formato de vector de vectores de u8 en el path definido en el móudlo de constantes llamado _PATH_SALIDA. Convierte a string cada uno de los elementos del tablero según las constantes definidas en el módulo de constantes.
/// Recibe el tablero en formato de vector de vectores de u8.
pub fn guardar(tablero: &Vec<Vec<u8>>) -> Result<(), ErrorArchivo> {
    let mut f = match crear(constantes::_PATH_SALIDA) {
        Ok(_f) => _f,
        Err(e) => return Err(e),
    };

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
            match escribir(&mut f, s) {
                Ok(()) => (),
                Err(e) => return Err(e),
            };
        }
        match escribir(&mut f, String::from(constantes::_ENTER_STR)) {
            Ok(()) => (),
            Err(e) => return Err(e),
        };
    }
    Ok(())
}

/// Traduce a bytes el string que le envían y lo escribe sobre el archivo que recibe.
/// Recibe el archivo a escribir y el string a traducir.
/// Entrega error en caso de no poder escribir sobre el archivo.
fn escribir(f: &mut File, s: String) -> Result<(), ErrorArchivo> {
    match f.write_all(s.as_bytes()) {
        Ok(()) => Ok(()),
        Err(_e) => Err(ErrorArchivo::Escritura(String::from(
            "No se puede escribir sobre el archivo de salida.",
        ))),
    }
}

/// Evalúa si se puede crear o no el archivo del path que recibe por parámetro.
fn crear(path: &str) -> Result<File, ErrorArchivo> {
    match File::create(path) {
        Ok(_f) => Ok(_f),
        Err(_e) => Err(ErrorArchivo::Creacion(String::from(
            "No se puede crear el archivo de salida.",
        ))),
    }
}
