use std::env;
use std::fs;

mod buscaminas;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mapa = fs::read_to_string(path).expect("[ERROR] Ingrese una ruta válida");

    buscaminas::main(&mapa);
}