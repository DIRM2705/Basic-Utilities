#ifndef MENU_BUILDER_H
#define MENU_BUILDER_H

typedef struct Menu
{
    char* name;
    char** options;
    unsigned int options_size;
} Menu;

Menu* new_menu(char* title);
void add_option(Menu* menu, char* option);
void print_menu(Menu* menu);

#endif