#include <stdio.h>


#define LOWER 0
#define UPPER 300
#define STEP 20


float ctof(float celsisu);


int main(int argc, char * argv[])
{
    float fahr, celsius;

    printf("--------------\n");
    printf("| cel | fahr |\n");
    printf("--------------\n");

    for(celsius = UPPER; celsius >= LOWER; celsius -= STEP) {
        fahr = ctof(celsius);
        printf("| %3.0f |  %3.0f |\n", celsius, fahr);
    }
    printf("--------------\n");

}


float ctof(float celsius)
{
    return celsius * (9.0 / 5.0 ) + 32;
}
