#include <stdio.h>

#define IN 0
#define OUT 1
#define MAX_WORD_LENGTH 10

int main(int argc, char *argv[])
{
    int state = OUT;
    char c;
    int word_length = 0;
    int word_hist[MAX_WORD_LENGTH] = {0};

    while ((c = getchar()) != EOF) {
        if (c == ' ' || c == '\t' || c == '\n') {
            if (state == IN) {
                if (word_length > MAX_WORD_LENGTH)
                    word_length = MAX_WORD_LENGTH;
                word_hist[word_length - 1]++;
                word_length = 0;
            }
            state = OUT;
        }
        else {
            word_length++;
            state = IN;
        }
    }

    for (int i = 0; i < MAX_WORD_LENGTH; i++) {
        printf("%2d: ", (i + 1));
        for (int j = 0; j < word_hist[i]; j++)
            printf("X");
        printf("\n");
    }
}
