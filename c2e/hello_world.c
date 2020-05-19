#include <stdio.h>

int main(int argc, char *argv[])
{
    printf("evaulation: `(getchar() != EOF)`\n\t result: %d\n",
            (getchar() != EOF));
    printf("EOF value is: %d\n", EOF);
}
