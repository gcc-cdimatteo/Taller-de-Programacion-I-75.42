pub fn main(data: &str){
    let data_asbytes = data.as_bytes();

    let (cant_filas, cant_columnas) = contar_filas_y_columnas(&data_asbytes);
    
    let _mapa = crear_mapa(&data_asbytes, cant_filas, cant_columnas);

}
 
fn contar_filas_y_columnas(data: &[u8]) -> (usize, usize) { // asumo q al final de cada fila hay un barra n
    let mut cant_filas = 0;
    let mut cant_columnas = 0;
    for i in data {
        if *i != 10 {
            if cant_filas == 0 { cant_columnas += 1; }
            continue;
        }
        cant_filas += 1; 
    }
    (cant_filas, cant_columnas - 1) // saco la ultima columna que es la del enter
}

fn crear_mapa(data: &[u8], cant_filas: usize, cant_columnas: usize) {
    
    let mut mapa = vec![vec![0; cant_columnas]; cant_filas];


    let mut fila = 0;
    let mut columna = 0;

    for i in data {
        mapa[fila][columna] = *i;
        println!("{:?}", mapa);

        if columna == cant_columnas { columna = 0; } else { columna += 1; }
        if *i == 10 { fila += 1; }
        println!("fila: {:?}", fila);
        println!("columna: {:?}", columna);
    }

    println!("{:?}", mapa);
}