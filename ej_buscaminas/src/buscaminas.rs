use std::str;

#[path = "../src/constantes.rs"]
mod constantes;

#[path = "../src/file.rs"]
mod file;

pub fn buscaminas(data: &str) -> Vec<Vec<u8>> {
    let data_asbytes = data.as_bytes();

    let (cant_filas, cant_columnas) = contar_filas_y_columnas(&data_asbytes);

    let mapa = crear_mapa(&data_asbytes, cant_filas, cant_columnas);

    let bombas = contar_bombas(&mapa, cant_filas, cant_columnas);

    let tablero = armar_tablero(&mapa, &bombas, cant_filas, cant_columnas);

    file::_guardar(&tablero);

    tablero
}

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

fn contar_bombas(mapa: &Vec<Vec<u8>>, cant_filas: usize, cant_columnas: usize) -> Vec<Vec<u8>> {
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

fn armar_tablero(
    mapa: &Vec<Vec<u8>>,
    bombas: &Vec<Vec<u8>>,
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
