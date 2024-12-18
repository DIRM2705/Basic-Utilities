import pro_reader

reader = pro_reader.ProReader("./Python examples/ProReader example/pro_reader.dll")
print(reader.read_int("int"))
print(reader.read_float("float"))
print(reader.read_string("string"))
print(reader.read_bool("bool"))