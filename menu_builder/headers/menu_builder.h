#ifndef MENU_BUILDER_H
#define MENU_BUILDER_H

#if __cplusplus
extern "C" {
#endif

typedef struct Menu
{
    char* name;
    char** options;
    unsigned int options_size;
} Menu;

Menu* new_menu(char* title);
void add_option(Menu* menu, char* option);
unsigned int read_option(Menu* menu);

#if __cplusplus
}
#endif

#endif