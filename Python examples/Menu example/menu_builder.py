import ctypes

class MenuBuilder:
    def __init__(self, path : str, menu_name : str):
        self.dll = ctypes.CDLL(path) #Carga la librería
        
        #Tipo de retorno y argumentos de new_menu
        self.dll.new_menu.restype = ctypes.POINTER(Menu)
        self.dll.new_menu.argtypes = [ctypes.c_char_p]
        
        #Tipos de argumentos de add_option
        self.dll.add_option.argtypes = [ctypes.POINTER(Menu), ctypes.c_char_p]
        
        #Tipo de retorno y argumentos de get_option
        self.dll.read_option.argtypes = [ctypes.POINTER(Menu)]
        self.dll.read_option.restype = ctypes.c_uint
        
        #Crea el menu
        self.menu = self.dll.new_menu(menu_name.encode())
    
    def add_option(self, option_name : str):
        self.dll.add_option(self.menu, option_name.encode())
    
    def read_option(self) -> int:
        return self.dll.read_option(self.menu)
        
class Menu(ctypes.Structure):
    _fields_ = [("name", ctypes.c_char_p),
                ("options_size", ctypes.c_uint),
                ("options", ctypes.POINTER(ctypes.c_char_p))]