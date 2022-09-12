window.SIDEBAR_ITEMS = {"fn":[["armar_tablero","Arma el tablero final, con la cantidad de bombas adyacentes a todas las cuadrículas del mapa y las bombas según corresponda. Donde una cuadrícula contenga el símbolo de bomba, significa que en el mapa había una bomba allí. Donde una cuadrícula contenga un número, representa la cantidad de bombas adyacentes a la misma, inclusive el 0 -es decir, que dicha cuadrícula no tiene bombas adyacentes-. Recibe el mapa original en formato de vector de vectores de u8, el mapa con la información de la cantidad de bombas adyacentes en formato de vector de vectores de u8, y la cantidad de filas y de columnas que contiene el mapa original -y por ende todos los demás tableros construidos-. Entrega un tablero en formato de vector de vectores de u8 con la cantidad de bombas adyacentes a todas las cuadrículas y las bombas del mapa."],["buscaminas","Función principal del procesamiento de traducción del mapa. Recibe la lectura del archivo enviado por el usuario en formato string. Construye y calcula las bombas adyacentes a todas las cuadrículas del mapa. Guarda en el archivo de salida en formato string el tablero con la información de las bombas adyacentes a todas las cuadrículas del mapa. Entrega un tablero en formato de vector de vectores de u8 con la información de las bombas adyacentes a todas las cuadrículas del mapa."],["contar_bombas","Cuenta la totalidad de bombas adyacentes a todas las cuadrículas, según la constante previamente definida en el módulo de constantes, existentes en un mapa. Recibe el mapa en formato de vector de vectores de u8, la cantidad de filas y columnas del mapa respectivamente. Entrega un tablero en formato de vector de vectores de u8 con la cantidad de bombas adyacentes a cada una de las cuadrículas del mapa. La cuadrícula que no tenga bombas adyacentes tendrá un 0 correspondientemente. La cuadrícula que contenga una bomba en su interior tendrá un 0."],["contar_filas_y_columnas","Cuenta la cantidad de filas y columnas de un arreglo de u8 que previamente se encontraba en formato string. Recibe el arreglo de u8. Entrega una tupla de dos elemntos de tipo usize que corresponden a la cantidad de filas y la cantidad de columnas que contiene el arreglo previamente convertido respectivamente."],["crear_mapa","Crea un mapa en formato de vector de vectores u8 a partir de un arreglo de u8. Recibe el arreglo de u8, la cantidad de filas y la cantidad de columnas que tendrá el mapa. Entrega el mapa como vector de vectores u8."]]};