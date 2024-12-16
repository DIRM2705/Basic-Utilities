#include "pro_reader.h"
#include <stdio.h>

void main() {
    int i = read_int("Enter an integer: ");
    printf("Integer: %d\n", i);
    float f = read_float("Enter a float: ");
    printf("Float: %f\n", f);
    char *s = read_string("Enter a string: ");
    printf("String: %s\n", s);
    short b = read_bool("Enter a bool: ");
    printf("Bool: %d\n", b);
}