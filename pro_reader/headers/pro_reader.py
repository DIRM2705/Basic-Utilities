import ctypes

class ProReader:
    def __init__(self, path : str):
        self.dll = ctypes.CDLL(path) #Carga la librería
        
        #Argumentos y tipo de retorno de read_int
        self.dll.read_int.argtypes = [ctypes.c_char_p]
        self.dll.read_int.restype = ctypes.c_int
        
        #Argumentos y tipo de retorno de read_float
        self.dll.read_float.argtypes = [ctypes.c_char_p]
        self.dll.read_float.restype = ctypes.c_float
        
        #Argumentos y tipo de retorno de read_string
        self.dll.read_string.argtypes = [ctypes.c_char_p]
        self.dll.read_string.restype = ctypes.c_char_p
        
        #Argumentos y tipo de retorno de read_bool
        self.dll.read_bool.argtypes = [ctypes.c_char_p]
        self.dll.read_bool.restype = ctypes.c_bool
        

    
    def read_int(self, instruction : str) -> int:
        return self.dll.read_int(instruction.encode())
    
    def read_float(self, instruction : str) -> float:
        return self.dll.read_float(instruction.encode())
    
    def read_string(self, instruction : str) -> str:
        return self.dll.read_string(instruction.encode()).decode()
    
    def read_bool(self, instruction : str) -> bool:
        return self.dll.read_bool(instruction.encode())
    
if __name__ == "__main__":
    reader = ProReader("target/debug/pro_reader.dll")
    #print(reader.read_int("int"))
    #print(reader.read_float("float"))
    print(reader.read_string("string"))
    #print(reader.read_bool("bool"))