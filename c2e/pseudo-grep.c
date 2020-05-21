/*
 * Exercise 4-1: implement grep-like program and print index of rightmost match
 */

#include <stdio.h>

#define MAXLINE 256     /* maximum input line length */

int get_line(char line[], int lim);
int strindex(char input[], char search_term[]);

int main(int argc, char *argv[])
{
    if (argc != 2) {
        printf("pseudo-grep\n\n\tUsage: pseudo-grep search_term\n");
        return -1;
    }

    char line[MAXLINE + 1];     /* leave extra char for null-termination */
    int search_hits = 0;
    int index;

    while (get_line(line, MAXLINE) > 0) {
        if ((index = strindex(line, argv[1])) >= 0) {
            printf("[%.2d]:%s", index, line);
            search_hits++;
        }
    }
    
    return search_hits;
}

int get_line(char line[], int lim)
{
    int c;
    int len = 0;

    while (len < MAXLINE && (c = getchar()) != EOF) {
        line[len++] = c;
        
        if (c == '\n')
            break;
    }

    line[len] = '\0';

    return len;
}

int strindex(char input[], char search_term[])
{
    int i, j, k;
    int index = -1;

    for (i = 0; input[i] != '\0'; i++) {
        for (j = i, k = 0; search_term[k] != '\0' && input[j] == search_term[k]; j++, k++)
            ;
        if (k > 0 && search_term[k] == '\0' && i > index)
            index = i;
    }

    return index;
}
