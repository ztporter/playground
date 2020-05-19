#include <stdio.h>

#define ASCII_LENGTH 128

int main(int argc, char* argv[])
{
    int char_counter[ASCII_LENGTH] = {0};
    char c;

    while ((c = getchar()) != EOF) {
        ++char_counter[c];
    }

    for (int i = 0; i < ASCII_LENGTH; i++) {
        if (char_counter[i] > 0) {
            printf("%c: ", (char) i);
            for (int j = 0; j < char_counter[i]; j++)
                printf("X");
            printf("\n");
        }
    }
}
