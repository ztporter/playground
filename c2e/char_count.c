#include <stdio.h>

int main(int argc, char *argv[])
{
    char c;
    int count = 0;
    char previous = '\n';

    while ((c = getchar()) != EOF) {
        if (c == ' ') {
            count++;
            if (previous != ' ')
                printf("echo: ' '\n");
        }
        else if (c == '\t') {
            count++;
            printf("echo: \\t\n");
        }
        else if (c == '\b') {
            printf("echo: \\b\n");
        }
        else if (c == '\\') {
            printf("echo: \\\n");
        }
        else if (c == '\n') {
            if (previous == '\n') {
                count++;
                printf("echo: \\n\n");
            }
        }
        else {
            printf("echo: %c\n", c);
        }
        previous = c;
    }
    printf("%d whitespace chars entered\n", count);
}
