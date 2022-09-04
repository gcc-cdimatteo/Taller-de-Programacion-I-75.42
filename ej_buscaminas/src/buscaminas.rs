pub fn main(data: &str) -> Vec<Vec<u8>> {

    let data_asbytes = data.as_bytes();

    let (cant_filas, cant_columnas) = contar_filas_y_columnas(&data_asbytes);

    let mapa = crear_mapa(&data_asbytes, cant_filas, cant_columnas);

    let bombas = contar_bombas(&mapa, cant_filas, cant_columnas);

    let tablero = armar_tablero(&mapa, &bombas, cant_filas, cant_columnas);

    traducir_mapa(&tablero); // -> escrbir en archivo

    tablero
}

fn contar_filas_y_columnas(data: &[u8]) -> (usize, usize) {
    let mut cant_filas = 0;
    let mut cant_columnas = 0;
    for i in data {
        if *i != 10 {
            if cant_filas == 0 {
                cant_columnas += 1;
            }
            continue;
        }
        cant_filas += 1;
    }
    (cant_filas + 1, cant_columnas) // sumo 1 fila por la ultima q no tiene barra n
}

fn crear_mapa(data: &[u8], cant_filas: usize, cant_columnas: usize) -> Vec<Vec<u8>> {
    let mut mapa = vec![vec![0; cant_columnas]; cant_filas];
    let mut fila = 0;
    let mut columna = 0;
    for i in data {
        if *i == 10 {
            fila += 1;
            continue;
        }
        mapa[fila][columna] = *i;
        if columna + 1 == cant_columnas {
            columna = 0;
        } else {
            columna += 1;
        }
    }
    mapa
}

fn _imprimir_mapa(mapa: &Vec<Vec<u8>>) {
    for i in mapa {
        for j in i {
            print!("{:?} ", j);
        }
        println!("");
    }
}

fn traducir_mapa(mapa: &Vec<Vec<u8>>) {
    for i in mapa {
        for j in i {
            if *j == 46 {
                print!(". ");
            } else if *j == 42 {
                print!("* ");
            } else {
                print!("{:?} ", j);
            }
        }
        println!("");
    }
}

fn contar_bombas(mapa: &Vec<Vec<u8>>, cant_filas: usize, cant_columnas: usize) -> Vec<Vec<u8>> {
    let mut bombas = vec![vec![0; cant_columnas]; cant_filas];
    for i in 0..cant_filas {
        for j in 0..cant_columnas {
            if mapa[i][j] == 42 {
                continue;
            } // actual
            if j + 1 != cant_columnas && mapa[i][j + 1] == 42 {
                bombas[i][j] += 1;
            } // derecha
            if j != 0 && mapa[i][j - 1] == 42 {
                bombas[i][j] += 1;
            } // izquierda
            if i + 1 != cant_filas && mapa[i + 1][j] == 42 {
                bombas[i][j] += 1;
            } // abajo
            if i != 0 && mapa[i - 1][j] == 42 {
                bombas[i][j] += 1;
            } // arriba
            if i + 1 != cant_filas && j + 1 != cant_columnas && mapa[i + 1][j + 1] == 42 {
                bombas[i][j] += 1;
            } // diagonal superior derecha
            if i + 1 != cant_filas && j != 0 && mapa[i + 1][j - 1] == 42 {
                bombas[i][j] += 1;
            } // diagonal superior izquierda
            if i != 0 && j + 1 != cant_columnas && mapa[i - 1][j + 1] == 42 {
                bombas[i][j] += 1;
            } // diagonal inferior derecha
            if i != 0 && j != 0 && mapa[i - 1][j - 1] == 42 {
                bombas[i][j] += 1;
            } // diagonal inferior izquierda
        }
    }
    bombas
}

fn armar_tablero(
    mapa: &Vec<Vec<u8>>,
    bombas: &Vec<Vec<u8>>,
    cant_filas: usize,
    cant_columnas: usize,
) -> Vec<Vec<u8>> {
    let mut tablero = vec![vec![46; cant_columnas]; cant_filas];
    for i in 0..cant_filas {
        for j in 0..cant_columnas {
            let cant_bombas = bombas[i][j];
            if mapa[i][j] == 42 {
                tablero[i][j] = 42;
            } else if cant_bombas != 0 {
                tablero[i][j] = cant_bombas;
            }
        }
    }
    tablero
}

#[cfg(test)]
mod tests {
    use super::*;

    // Unitarias

    #[test]
    fn contar_matriz_2x2() {
        assert_eq!(contar_filas_y_columnas(&[1, 1, 10, 1, 1]), (2,2));
    }

    // Integracion

    #[test]
    fn buscaminas_1() {
        assert_eq!(main(&".*.*.\n..*..\n..*..\n....."), [[1, 42, 3, 42, 1], [1, 3, 42, 3, 1], [46, 2, 42, 2, 46], [46, 1, 1, 1, 46]]);
    }

    #[test]
    fn buscaminas_2() {
        assert_ne!(main(&"**.*.\n..*..\n..*..\n....."), [[1, 42, 3, 42, 1], [1, 3, 42, 3, 1], [46, 2, 42, 2, 46], [46, 1, 1, 1, 46]]);
    }

    #[test]
    fn buscaminas_3() {
        assert_eq!(main(&"**.*.\n..*..\n..*..\n....."), [[42, 42, 3, 42, 1], [2, 4, 42, 3, 1], [46, 2, 42, 2, 46], [46, 1, 1, 1, 46]]);
    }

}