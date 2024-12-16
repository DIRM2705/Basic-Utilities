#include "menu_builder.h"

int main() {
    Menu* menu = new_menu("Main Menu");
    add_option(menu, "Option 1");
    add_option(menu, "Option 2");
    add_option(menu, "Option 3");

    unsigned int opc = read_option(menu);

    printf("Option selected: %d\n", opc);
    return 0;
}