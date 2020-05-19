#include <stdio.h>

int main(int argc, char * argv[])
{
    float fahr, celsius;
    int lower, upper, step;

    lower = 0;
    upper = 300;
    step = 20;

    printf("----------------\n");
    printf("| fahr |  cel  |\n");
    printf("----------------\n");

    for(fahr = lower; fahr <= upper; fahr += step) {
        celsius = 5 * (fahr - 32) / 9;
        printf("|  %3.0f |%6.1f |\n", fahr, celsius);
    }
    printf("----------------\n");

}
