#include <stdio.h>


#define MAX_LINE 256


int get_line(char line[]);


int main(int argc, char *argv[])
{
    int len;
    char line[MAX_LINE + 1];    /* leave a byte for the null terminator */

    while ((len = get_line(line)) > 0) {
        printf("[%2d]:%s", len, line) /* comment */ && printf("continue\n");
    }
    return 0;
}

int get_line(char line[])
{
    char c;
    int len = 0;

    while ((c = getchar()) != EOF) {
        line[len++] = c;

        if ((len == MAX_LINE) || (c == '\n')) {
            break;
        }
    }

    line[len] = '\0';

    return len;
}
