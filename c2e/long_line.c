#include <stdio.h>


#define MAX_LINE 256
#define PRINT_CUTOFF 40


int get_line(char line[], int maxline);
void terminate_line(char line[], int size, int max_size);
int count_trailing_whitespace(char line[], int size, int max_size);
void copy(char to[], char from[]);
void reverse(char line[], int size, int max_size);


int main(int argc, char *argv[])     
{
    int len;
    char line[MAX_LINE];

    while ((len = get_line(line, MAX_LINE)) > 0) {
        if (line[0] != '\n') {
            reverse(line, len, MAX_LINE);
            printf("[%3d]: %s", len, line);
        }
    }
    return 0;
}


int get_line(char line[], int char_limit)
{
    char c;
    int num_chars = 0;
    int wspace;

    while (((c = getchar()) != EOF) && (c != '\n')) {
        if (num_chars < char_limit)
            line[num_chars] = c;
        num_chars++;
    }

    wspace = count_trailing_whitespace(line, num_chars, char_limit);
    num_chars = num_chars - wspace;
    terminate_line(line, num_chars, char_limit);

    if (c == '\n')
        num_chars++;

    return num_chars;
}


void terminate_line(char line[], int size, int max_size)
{
    int nl_index, null_index;

    if (size > (max_size - 2)) {
        nl_index = max_size - 2;
        null_index = max_size - 1;
    }
    else {
        nl_index = size;
        null_index = size + 1;
    }
    line[nl_index] = '\n';
    line[null_index] = '\0';
}


int count_trailing_whitespace(char line[], int size, int max_size)
{
    int index;
    int whitespace_count = 0;
    if (size > max_size) {
        index = max_size - 1;
    }
    else {
        index = size - 1;
    }
    
    while (index >= 0 && (line[index] == ' ' || line[index] == '\t')) {
        whitespace_count++;
        index--;
    }
    return whitespace_count;
}


void copy(char to[], char from[])
{
    for (int i = 0; (to[i] = from[i]) != '\0'; i++);
}


void reverse(char line[], int size, int max_size)
{
    char temp[max_size];
    int index;
    if (size > (max_size - 2)) {
        index = max_size - 2;
    }
    else {
        index = size - 2;
    }

    for (int i = 0; i < (size - 1); i++) {
        temp[i] = line[index];
        index--;
    }

    temp[size - 1] = '\n';
    temp[size] = '\0';

    copy(line, temp);
}
