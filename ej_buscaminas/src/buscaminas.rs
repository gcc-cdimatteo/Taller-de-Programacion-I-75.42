use std::str;

pub fn main(data: &str) -> Vec<Vec<u8>> {

    let data_asbytes = data.as_bytes();

    let (cant_filas, cant_columnas) = contar_filas_y_columnas(&data_asbytes);

    let mapa = crear_mapa(&data_asbytes, cant_filas, cant_columnas);

    let bombas = contar_bombas(&mapa, cant_filas, cant_columnas);

    let tablero = armar_tablero(&mapa, &bombas, cant_filas, cant_columnas);

    traducir_mapa(&tablero); // -> escrbir en archivo

    for i in &tablero {
        println!("t: {:?}", i);
        
        let s = match str::from_utf8(i) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    
        println!("result: {}", s);
    };

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

    #[test]
    fn contar_matriz_3x4() {
        assert_eq!(contar_filas_y_columnas(&[1, 1, 1, 1, 10, 1, 1, 1, 1, 10, 1, 1, 1, 1]), (3,4));
    }

    #[test]
    fn crear_mapa_2x2() {
        assert_eq!(crear_mapa(&[1, 1, 10, 1, 1], 2, 2), [[1, 1], [1, 1]]);
    }

    #[test]
    fn crear_mapa_3x4() {
        assert_eq!(crear_mapa(&[1, 1, 1, 1, 10, 1, 1, 1, 1, 10, 1, 1, 1, 1], 3, 4), [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]);
    }

    #[test]
    fn contar_bombas_1() {
        let punto: u8 = 46;
        let bomba: u8 = 42;
        assert_eq!(contar_bombas(&vec![vec![punto, bomba], vec![punto, punto]], 2, 2), [[1, 0], [1, 1]]);
    }

    #[test]
    fn contar_bombas_11() {
        let punto: u8 = 46;
        let bomba: u8 = 42;
        assert_eq!(contar_bombas(
            &vec![
                vec![punto, punto, bomba, punto, bomba, bomba, punto, punto, punto], 
                vec![bomba, punto, punto, punto, punto, bomba, punto, bomba, punto], 
                vec![punto, punto, bomba, punto, punto, punto, punto, bomba, punto], 
                vec![bomba, punto, punto, punto, punto, bomba, punto, punto, bomba]
            ], 4, 9), 
            [
                [1, 2, 0, 2, 0, 0, 3, 1, 1], 
                [0, 3, 2, 3, 3, 0, 4, 0, 2], 
                [2, 3, 0, 1, 2, 2, 4, 0, 3], 
                [0, 2, 1, 1, 1, 0, 2, 2, 0]
            ]
            );
    }

    #[test]
    fn armar_tablero_1() {
        let punto: u8 = 46;
        let bomba: u8 = 42;
        let mapa: Vec<Vec<u8>> = vec![vec![punto, bomba], vec![punto, punto]];
        let bombas: Vec<Vec<u8>> = vec![vec![1, 0], vec![1, 1]];
        assert_eq!(armar_tablero(&mapa, &bombas, 2, 2), [[1, 42], [1, 1]]);
    }

    #[test]
    fn armar_tablero_11() {
        let punto: u8 = 46;
        let bomba: u8 = 42;
        let mapa: Vec<Vec<u8>> = vec![
            vec![punto, punto, bomba, punto, bomba, bomba, punto, punto, punto], 
            vec![bomba, punto, punto, punto, punto, bomba, punto, bomba, punto], 
            vec![punto, punto, bomba, punto, punto, punto, punto, bomba, punto], 
            vec![bomba, punto, punto, punto, punto, bomba, punto, punto, bomba]
        ];
        let bombas: Vec<Vec<u8>> = vec![
            vec![1, 2, 0, 2, 0, 0, 3, 1, 1], 
            vec![0, 3, 2, 3, 3, 0, 4, 0, 2], 
            vec![2, 3, 0, 1, 2, 2, 4, 0, 3], 
            vec![0, 2, 1, 1, 1, 0, 2, 2, 0]
        ];

        assert_eq!(armar_tablero(&mapa, &bombas, 4, 9), 
            vec![
                vec![1, 2, 42, 2, 42, 42, 3, 1, 1], 
                vec![42, 3, 2, 3, 3, 42, 4, 42, 2], 
                vec![2, 3, 42, 1, 2, 2, 4, 42, 3], 
                vec![42, 2, 1, 1, 1, 42, 2, 2, 42]
            ]
        );

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