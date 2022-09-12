#[path = "../src/constantes.rs"]
mod constantes;

#[path = "../src/buscaminas.rs"]
mod buscaminas;

#[test]
fn buscaminas_1() {
    let mapa = ".*.*.\n..*..\n..*..\n.....";
    assert_eq!(buscaminas::buscaminas(mapa), [[1, constantes::_BOMBA_U8, 3, constantes::_BOMBA_U8, 1], [1, 3, constantes::_BOMBA_U8, 3, 1], [constantes::_VACIO_U8, 2, constantes::_BOMBA_U8, 2, constantes::_VACIO_U8], [constantes::_VACIO_U8, 1, 1, 1, constantes::_VACIO_U8]]);
}

#[test]
fn buscaminas_2() {
    let mapa = "**.*.\n..*..\n..*..\n.....";
    assert_ne!(buscaminas::buscaminas(mapa), [[1, constantes::_BOMBA_U8, 3, constantes::_BOMBA_U8, 1], [1, 3, constantes::_BOMBA_U8, 3, 1], [constantes::_VACIO_U8, 2, constantes::_BOMBA_U8, 2, constantes::_VACIO_U8], [constantes::_VACIO_U8, 1, 1, 1, constantes::_VACIO_U8]]);
}

#[test]
fn buscaminas_3() {
    let mapa = "**.*.\n..*..\n..*..\n.....";
    assert_eq!(buscaminas::buscaminas(mapa), [[constantes::_BOMBA_U8, constantes::_BOMBA_U8, 3, constantes::_BOMBA_U8, 1], [2, 4, constantes::_BOMBA_U8, 3, 1], [constantes::_VACIO_U8, 2, constantes::_BOMBA_U8, 2, constantes::_VACIO_U8], [constantes::_VACIO_U8, 1, 1, 1, constantes::_VACIO_U8]]);
}
