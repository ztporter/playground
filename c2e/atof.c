/*
 * Exercise 4-2: implement atof function that can handle scientific notation
 */

#include <ctype.h>
#include <math.h>
#include <stdio.h>

double atof(char s[]);

int main(int argc, char* argv[])
{
    if (argc != 2) {
        printf("atof\n\n\tUsage:\tatof float_string\n");
        return -1;
    }

    printf("%f\n", atof(argv[1]));
    return 0;
}

double atof(char s[])
{
    double val, power, exp;
    int i, sign, exp_sign;

    for (i = 0; isspace(s[i]); i++)
        ;

    sign = (s[i] == '-') ? -1 : 1;
    if (s[i] == '-' || s[i] == '+')
        i++;

    for (val = 0.0; isdigit(s[i]); i++)
        val = 10.0 * val + (s[i] - '0');

    if (s[i] == '.')
        i++;

    for (power = 1.0; isdigit(s[i]); i++) {
        val = 10.0 * val + (s[i] - '0');
        power *= 10.0;
    }

    val = sign * val / power;

    if (s[i] == 'e' || s[i] == 'E') {
        i++;

        exp_sign = (s[i] == '-') ? -1 : 1;
        if (s[i] == '-' || s[i] == '+')
            i++;

        for (exp = 0.0; isdigit(s[i]); i++)
            exp = 10.0 * exp + (s[i] - '0');

        exp = exp_sign * exp;

        val = val * pow(10, exp);
    }

    return val;
}
