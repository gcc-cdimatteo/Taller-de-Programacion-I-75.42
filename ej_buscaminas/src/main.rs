use std::env;

mod usr;
mod file;
mod buscaminas;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    usr::entrada();
    
    usr::procesamiento(path);

    buscaminas::buscaminas(&file::_leer(path));

    usr::salida();

}