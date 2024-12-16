import menu_builder

menu = MenuBuilder("../BasicUtilitiesLibs/menu_builder.dll", "Main Menu")
menu.add_option("Option 1")
menu.add_option("Option 2")
menu.add_option("Option 3")
menu.add_option("Option 4")

option = menu.read_option()