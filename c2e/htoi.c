#include <stdio.h>


int htoi(char s[]);
int ctoi(char c);


int main(int argc, char * argv[])
{
    if (argc != 2) {
        printf("Usage: htoi hex_string\n");
        return -1;
    }

    printf("%d\n", htoi(argv[1]));
    return 0;
}


int htoi(char s[])
{
    int i = 0;
    int num = 0;
    char c;

    if ((s[0] == '0') && ((s[1] == 'x') || (s[1] == 'X')))
        i = 2;

    while (s[i] != '\0') {
        num = num * 16 + ctoi(s[i]);
        i++;
    }

    return num;
}


int ctoi(char c)
{
    if ((c >= '0') && (c <= '9')) {
        return c - '0';
    }
    else {
        return c - 'A' + 10;
    }
}
