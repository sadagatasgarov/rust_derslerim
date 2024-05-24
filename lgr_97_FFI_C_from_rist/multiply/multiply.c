#include <stdio.h>
#include <stdint.h>

int multiply(int a, int b) {
    printf("[C] Hello from C!\n");
    printf("[C] Input a is: %i \n", a);
    printf("[C] Input b is: %i \n", b);
    printf("[C] Multiplying and returning result to Rust..\n");

    return a * b;
}