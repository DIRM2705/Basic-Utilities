# Basic Utilities

### Introducción

Basic utilities es un conjunto de bibliotecas diseñados para facilitar las tareas monotonas que los estudiantes enfrentamos al estudiar programación. Utiliza librerías dinámicas (dll) para proveer interfaces que permitan la integración de funciones en distintos lenguajes. Actualmente, hay implementaciones para C, C++ y Python. Adicionalmente, se espera agregar un soporte más sencillo para Rust en un futuro cercano.

### Instalación

1. Elige la librería o librerías que deseas agregar a tu proyecto.

2. Ve a la carpeta de la librería y ubica la subcarpeta *headers*

![carpeta pro_reader](https://imgur.com/NNdpkFY.png)

3. Dentro de headers, descarga el archivo para tu lenguaje (*.h si es C o C++, *.py para Python)

![headers pro_reader](https://imgur.com/yKuYmj2.png)

4. Ve a la sección de [releases](https://github.com/DIRM2705/Basic-Utilities/releases) de este repositorio

![seccion releases](https://imgur.com/32IC2VX.png)

5. Descarga el dll de la librería que desees agregar. Puede que veas varias versiones, recomiendo que descargues la última.

6. Crea tu proyecto. En una carpeta crea tu codigo fuente, agrega el header y el dll.

![carpeta proyecto](https://imgur.com/9nU1aow.png)

7. Incluye el header a tu código fuente

Para C/C++:
```C
#include "lib.h"

void main()
{
    //Tu código va aquí
}
```

Para Python:
```Py
import lib

#Tu códgio va aquí
```
**Nota**
Reemplaza lib por el nombre de la librería que estás agregando

8. Si estás usando C o C++ y el compilador gcc, puedes usar la siguiente linea de comando para compilar tu proyecto.

```bash
gcc -o main tu_archivo.c libreria1.dll libreria2.dll ... librerian.dll
```
**Nota**
Reemplaza tu_archivo.c por tu archivo de codigo fuente y libreria1.dll, libreria2.dll ... librerian.dll por cada una de las librerías que utilizarás.

### Uso

Puedes ver las funciones de cada librería y su uso en los siguientes enlaces

* [Menu builder](https://github.com/DIRM2705/Basic-Utilities/tree/main/menu_builder#readme)
* [Pro reader]()

### Créditos
* [Daniel Rosas](https://github.com/DIRM2705)