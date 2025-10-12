import menu_builder

menu = menu_builder.MenuBuilder("./Python examples/Menu example/menu_builder.dll", "Main Menu")
menu.add_option("Option 1")
menu.add_option("Option 2")
menu.add_option("Option 3")
menu.add_option("Option 4")

option = menu.read_option()
print("Option selected: " + str(option))