### Funciones de Menu Builder

#### Documentación para C

##### Estructuras

`struct Menu`

Campos:

* `char* name` El título del menú
* `char** options` Un arreglo que contiene todas las opciones del menú
* `unsigned int options_size` El número de opciones que contiene el menú

##### Funciones

`new_menu(char* title)`

Inicializa la estructura de Menu con su título y cero opciones

**ARGS**

`char* title` El título del menú

**RETURNS**

`Menu *` Un apuntador al menú inicializado

<hr>

`add_option(Menu* menu, char* option)`

Añade una opción al menú especificado

**ARGS**

`Menu* menu` El menú al que se le agregará una opción

`char* option` Una cadena que representa la opción a agregar

<hr>

`read_option(Menu* menu)`

Utiliza el standard input stream para leer una opción del usuario y validarla

**ARGS**

`Menu* menu` El menú de opciones que se mostrará

**RETURNS**

`unsigned int` Un número que representa la opción elegida

#### Documentación para Python

##### Constructor

`class MenuBuilder(path, menu_name)`

Crea un objeto de tipo MenuBuilder

**ARGS:**

`str : path` La ruta donde se encuentra el dll de la librería siguiendo el patrón `bar\foo\lib.dll`

`str : menu_name` El título que mostrará el menú

##### Métodos

`MenuBuilder.add_option(option)`

Agrega una opción al menú

**ARGS:**

`str : option` La opción a agregar en el menú

<hr>

`MenuBuilder.read_option()`

Utiliza el standard input stream para obtener una entrada del usuario

**RETURNS**

`int` Un entero representando el número de opción elegido

