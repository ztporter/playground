#include <stdio.h>


#define COLUMNS_PER_TAB 4
#define MAX_LINE 80


int get_line(char line[]);
void terminate_line(char line[], int size);
void add_spaces_for_tab(char line[], int *size);


int main(int argc, char *argv[])
{
    int len;
    char line[MAX_LINE + 2];

    while ((len = get_line(line)) > 0) {
        printf("[%3d]: %s", len, line);
    }
    return 0;
}


int get_line(char line[])
{
    char c;
    int num_chars = 0;

    while (((c = getchar()) != EOF) && (c != '\n')) {
        if (c != '\t') {
            line[num_chars++] = c;
        }
        else {
            add_spaces_for_tab(line, &num_chars);
        }

        if (num_chars == MAX_LINE)
            break;
    }

    terminate_line(line, num_chars);

    if (c == '\n')
        num_chars++;

    return num_chars;
}


void terminate_line(char line[], int size)
{
    int nl_index, null_index;

    if (size > MAX_LINE) {
        nl_index = MAX_LINE;
        null_index = MAX_LINE + 1;
    }
    else {
        nl_index = size;
        null_index = size + 1;
    }
    line[nl_index] = '\n';
    line[null_index] = '\0';
}


void add_spaces_for_tab(char line[], int *size)
{
    do {
        line[(*size)++] = ' ';
    }
    while (((*size) % COLUMNS_PER_TAB) != 0);
}
