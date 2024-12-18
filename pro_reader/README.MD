### Funciones de Pro Reader

#### Documentación para C

`read_int(const char* instruction)`

Obtiene una entrada del usuario y valida que sea un número entero

**ARGS**

`const char* instruction` Una cadena que sirve como indicación para el usuario

**RETURNS**

`int` Un número entero ingresado por el usuario

<hr>

`read_float(const char* instruction)`

Obtiene una entrada del usuario y valida que sea un número real

**ARGS**

`const char* instruction` Una cadena que sirve como indicación para el usuario

**RETURNS**

`float` Un número real ingresado por el usuario

<hr>

`read_string(const char* instruction)`

Obtiene una entrada del usuario y lo convierte en una cadena de caracteres de manera segura

**ARGS**

`const char* instruction` Una cadena que sirve como indicación para el usuario

**RETURNS**

`char *` Un apuntador que representa un arreglo de caracteres

<hr>

`read_bool(const char * instruction)`

Obtiene una entrada del usuario y valida que sea un booleano

**ARGS**

`const char* instruction` Una cadena que sirve como indicación para el usuario

**RETURNS**

`short` 0 para falso, 1 para verdadero según lo ingresado por el usuario

#### Documentación para Python

##### Constructor

`class ProReader(path)`

Crea un objeto de la clase ProReader

**ARGS**

`str : path` La ruta donde se encuentra el dll de la librería siguiendo el patrón `bar\foo\lib.dll`

##### Métodos

`ProReader.read_int(instruction)`

Obtiene una entrada del usuario y valida que sea un entero

**ARGS**

`str : instruction` Una cadena que sirve como instrucción para el usuario

**RETURNS**

`int` Un entero ingresado por el usuario

<hr>

`ProReader.read_float(instruction)`

Obtiene una entrada del usuario y valida que sea un real

**ARGS**

`str : instruction` Una cadena que sirve como instrucción para el usuario

**RETURNS**

`float` Un real ingresado por el usuario

<hr>

`ProReader.read_string(instruction)`

Obtiene una entrada del usuario y la convierte en una cadena de manera segura

**ARGS**

`str : instruction` Una cadena que sirve como instrucción para el usuario

**RETURNS**

`str` Una cadena ingresada por el usuario

<hr>

`ProReader.read_bool(instruction)`

Obtiene una entrada del usuario y valida que sea un booleano

**ARGS**

`str : instruction` Una cadena que sirve como instrucción para el usuario

**RETURNS**

`bool` Un valor booleano ingresado por el usuario