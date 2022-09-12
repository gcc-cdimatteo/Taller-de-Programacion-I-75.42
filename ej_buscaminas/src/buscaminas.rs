use std::str;

use crate::constantes;
use crate::file;
use crate::file::ErrorArchivo;

/// Función principal del procesamiento de traducción del mapa.
/// Recibe la lectura del archivo enviado por el usuario en formato string.
/// Construye y calcula las bombas adyacentes a todas las cuadrículas del mapa.
/// Guarda en el archivo de salida en formato string el tablero con la información de las bombas adyacentes a todas las cuadrículas del mapa.
/// Entrega un tablero en formato de vector de vectores de u8 con la información de las bombas adyacentes a todas las cuadrículas del mapa.
pub fn buscaminas(data: &str) -> Result<Vec<Vec<u8>>, ErrorArchivo> {
    let data_asbytes = data.as_bytes();

    let (cant_filas, cant_columnas) = contar_filas_y_columnas(data_asbytes);

    let mapa = crear_mapa(data_asbytes, cant_filas, cant_columnas);

    let bombas = contar_bombas(&mapa, cant_filas, cant_columnas);

    let tablero = armar_tablero(&mapa, &bombas, cant_filas, cant_columnas);

    match file::guardar(&tablero) {
        Ok(()) => Ok(tablero),
        Err(e) => Err(e),
    }
}

/// Cuenta la cantidad de filas y columnas de un arreglo de u8 que previamente se encontraba en formato string.
/// Recibe el arreglo de u8.
/// Entrega una tupla de dos elemntos de tipo usize que corresponden a la cantidad de filas y la cantidad de columnas que contiene el arreglo previamente convertido respectivamente.
fn contar_filas_y_columnas(data: &[u8]) -> (usize, usize) {
    let mut cant_filas = 0;
    let mut cant_columnas = 0;
    for i in data {
        if *i != constantes::_ENTER_U8 {
            if cant_filas == 0 {
                cant_columnas += 1;
            }
            continue;
        }
        cant_filas += 1;
    }
    (cant_filas + 1, cant_columnas) // sumo 1 fila por la ultima q no tiene barra n
}

/// Crea un mapa en formato de vector de vectores u8 a partir de un arreglo de u8.
/// Recibe el arreglo de u8, la cantidad de filas y la cantidad de columnas que tendrá el mapa.
/// Entrega el mapa como vector de vectores u8.
fn crear_mapa(data: &[u8], cant_filas: usize, cant_columnas: usize) -> Vec<Vec<u8>> {
    let mut mapa = vec![vec![0; cant_columnas]; cant_filas];
    let mut fila = 0;
    let mut columna = 0;
    for i in data {
        if *i == constantes::_ENTER_U8 {
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

/// Cuenta la totalidad de bombas adyacentes a todas las cuadrículas, según la constante previamente definida en el módulo de constantes, existentes en un mapa.
/// Recibe el mapa en formato de vector de vectores de u8, la cantidad de filas y columnas del mapa respectivamente.
/// Entrega un tablero en formato de vector de vectores de u8 con la cantidad de bombas adyacentes a cada una de las cuadrículas del mapa. La cuadrícula que no tenga bombas adyacentes tendrá un 0 correspondientemente. La cuadrícula que contenga una bomba en su interior tendrá un 0.
fn contar_bombas(mapa: &[Vec<u8>], cant_filas: usize, cant_columnas: usize) -> Vec<Vec<u8>> {
    let mut bombas = vec![vec![0; cant_columnas]; cant_filas];
    for i in 0..cant_filas {
        for j in 0..cant_columnas {
            if mapa[i][j] == constantes::_BOMBA_U8 {
                continue;
            } // actual
            if j + 1 != cant_columnas && mapa[i][j + 1] == constantes::_BOMBA_U8 {
                bombas[i][j] += 1;
            } // derecha
            if j != 0 && mapa[i][j - 1] == constantes::_BOMBA_U8 {
                bombas[i][j] += 1;
            } // izquierda
            if i + 1 != cant_filas && mapa[i + 1][j] == constantes::_BOMBA_U8 {
                bombas[i][j] += 1;
            } // abajo
            if i != 0 && mapa[i - 1][j] == constantes::_BOMBA_U8 {
                bombas[i][j] += 1;
            } // arriba
            if i + 1 != cant_filas
                && j + 1 != cant_columnas
                && mapa[i + 1][j + 1] == constantes::_BOMBA_U8
            {
                bombas[i][j] += 1;
            } // diagonal superior derecha
            if i + 1 != cant_filas && j != 0 && mapa[i + 1][j - 1] == constantes::_BOMBA_U8 {
                bombas[i][j] += 1;
            } // diagonal superior izquierda
            if i != 0 && j + 1 != cant_columnas && mapa[i - 1][j + 1] == constantes::_BOMBA_U8 {
                bombas[i][j] += 1;
            } // diagonal inferior derecha
            if i != 0 && j != 0 && mapa[i - 1][j - 1] == constantes::_BOMBA_U8 {
                bombas[i][j] += 1;
            } // diagonal inferior izquierda
        }
    }
    bombas
}

/// Arma el tablero final, con la cantidad de bombas adyacentes a todas las cuadrículas del mapa y las bombas según corresponda.
/// Donde una cuadrícula contenga el símbolo de bomba, significa que en el mapa había una bomba allí. Donde una cuadrícula contenga un número, representa la cantidad de bombas adyacentes a la misma, inclusive el 0 -es decir, que dicha cuadrícula no tiene bombas adyacentes-.
/// Recibe el mapa original en formato de vector de vectores de u8, el mapa con la información de la cantidad de bombas adyacentes en formato de vector de vectores de u8, y la cantidad de filas y de columnas que contiene el mapa original -y por ende todos los demás tableros construidos-.
/// Entrega un tablero en formato de vector de vectores de u8 con la cantidad de bombas adyacentes a todas las cuadrículas y las bombas del mapa.
fn armar_tablero(
    mapa: &[Vec<u8>],
    bombas: &[Vec<u8>],
    cant_filas: usize,
    cant_columnas: usize,
) -> Vec<Vec<u8>> {
    let mut tablero = vec![vec![constantes::_VACIO_U8; cant_columnas]; cant_filas];
    for i in 0..cant_filas {
        for j in 0..cant_columnas {
            let cant_bombas = bombas[i][j];
            if mapa[i][j] == constantes::_BOMBA_U8 {
                tablero[i][j] = constantes::_BOMBA_U8;
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

    #[test]
    fn contar_matriz_2x2() {
        assert_eq!(
            contar_filas_y_columnas(&[1, 1, constantes::_ENTER_U8, 1, 1]),
            (2, 2)
        );
    }

    #[test]
    fn contar_matriz_3x4() {
        assert_eq!(
            contar_filas_y_columnas(&[
                1,
                1,
                1,
                1,
                constantes::_ENTER_U8,
                1,
                1,
                1,
                1,
                constantes::_ENTER_U8,
                1,
                1,
                1,
                1
            ]),
            (3, 4)
        );
    }

    #[test]
    fn crear_mapa_2x2() {
        assert_eq!(
            crear_mapa(&[1, 1, constantes::_ENTER_U8, 1, 1], 2, 2),
            [[1, 1], [1, 1]]
        );
    }

    #[test]
    fn crear_mapa_3x4() {
        assert_eq!(
            crear_mapa(
                &[
                    1,
                    1,
                    1,
                    1,
                    constantes::_ENTER_U8,
                    1,
                    1,
                    1,
                    1,
                    constantes::_ENTER_U8,
                    1,
                    1,
                    1,
                    1
                ],
                3,
                4
            ),
            [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        );
    }

    #[test]
    fn contar_bombas_1() {
        assert_eq!(
            contar_bombas(
                &vec![
                    vec![constantes::_VACIO_U8, constantes::_BOMBA_U8],
                    vec![constantes::_VACIO_U8, constantes::_VACIO_U8]
                ],
                2,
                2
            ),
            [[1, 0], [1, 1]]
        );
    }

    #[test]
    fn contar_bombas_11() {
        assert_eq!(
            contar_bombas(
                &vec![
                    vec![
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_BOMBA_U8,
                        constantes::_VACIO_U8,
                        constantes::_BOMBA_U8,
                        constantes::_BOMBA_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8
                    ],
                    vec![
                        constantes::_BOMBA_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_BOMBA_U8,
                        constantes::_VACIO_U8,
                        constantes::_BOMBA_U8,
                        constantes::_VACIO_U8
                    ],
                    vec![
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_BOMBA_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_BOMBA_U8,
                        constantes::_VACIO_U8
                    ],
                    vec![
                        constantes::_BOMBA_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_BOMBA_U8,
                        constantes::_VACIO_U8,
                        constantes::_VACIO_U8,
                        constantes::_BOMBA_U8
                    ]
                ],
                4,
                9
            ),
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
        let mapa: Vec<Vec<u8>> = vec![
            vec![constantes::_VACIO_U8, constantes::_BOMBA_U8],
            vec![constantes::_VACIO_U8, constantes::_VACIO_U8],
        ];
        let bombas: Vec<Vec<u8>> = vec![vec![1, 0], vec![1, 1]];
        assert_eq!(
            armar_tablero(&mapa, &bombas, 2, 2),
            [[1, constantes::_BOMBA_U8], [1, 1]]
        );
    }

    #[test]
    fn armar_tablero_11() {
        let mapa: Vec<Vec<u8>> = vec![
            vec![
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_BOMBA_U8,
                constantes::_VACIO_U8,
                constantes::_BOMBA_U8,
                constantes::_BOMBA_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
            ],
            vec![
                constantes::_BOMBA_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_BOMBA_U8,
                constantes::_VACIO_U8,
                constantes::_BOMBA_U8,
                constantes::_VACIO_U8,
            ],
            vec![
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_BOMBA_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_BOMBA_U8,
                constantes::_VACIO_U8,
            ],
            vec![
                constantes::_BOMBA_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_BOMBA_U8,
                constantes::_VACIO_U8,
                constantes::_VACIO_U8,
                constantes::_BOMBA_U8,
            ],
        ];
        let bombas: Vec<Vec<u8>> = vec![
            vec![1, 2, 0, 2, 0, 0, 3, 1, 1],
            vec![0, 3, 2, 3, 3, 0, 4, 0, 2],
            vec![2, 3, 0, 1, 2, 2, 4, 0, 3],
            vec![0, 2, 1, 1, 1, 0, 2, 2, 0],
        ];

        assert_eq!(
            armar_tablero(&mapa, &bombas, 4, 9),
            vec![
                vec![
                    1,
                    2,
                    constantes::_BOMBA_U8,
                    2,
                    constantes::_BOMBA_U8,
                    constantes::_BOMBA_U8,
                    3,
                    1,
                    1
                ],
                vec![
                    constantes::_BOMBA_U8,
                    3,
                    2,
                    3,
                    3,
                    constantes::_BOMBA_U8,
                    4,
                    constantes::_BOMBA_U8,
                    2
                ],
                vec![
                    2,
                    3,
                    constantes::_BOMBA_U8,
                    1,
                    2,
                    2,
                    4,
                    constantes::_BOMBA_U8,
                    3
                ],
                vec![
                    constantes::_BOMBA_U8,
                    2,
                    1,
                    1,
                    1,
                    constantes::_BOMBA_U8,
                    2,
                    2,
                    constantes::_BOMBA_U8
                ]
            ]
        );
    }
}
