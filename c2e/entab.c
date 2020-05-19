#include <stdio.h>


#define TAB_LENGTH 4
#define MAX_LINE 80


int get_line(char line[]);
void terminate_line(char line[], int size);
void entab_line(char line[]);


int main(int argc, char *argv[])
{
    int len;
    char line[MAX_LINE + 1];

    while ((len = get_line(line)) > 0) {
        printf("[notab]:\n%s", line);
        entab_line(line);
        printf("[entab]:\n%s", line);
    }
    return 0;
}


int get_line(char line[])
{
    char c;
    int num_chars = 0;

    while ((c = getchar()) != EOF) {
        line[num_chars++] = c;

        if ((num_chars == MAX_LINE) || (c == '\n'))
            break;
    }

    line[num_chars] = '\0';

    return num_chars;
}


void entab_line(char line[])
{
    char temp[MAX_LINE + 1];
    char c;
    int nspaces = 0;
    int index = 0;

    for (int i = 0; (c = line[i]) != '\0'; i++) {
        if (c == ' ') {
            nspaces++;
            if (((i + 1) % TAB_LENGTH) == 0) {
                if (nspaces == 1) {
                    temp[index++] = ' ';
                }
                else {
                    temp[index++] = '\t';
                }
                nspaces = 0;
            }
        }
        else {
            while (nspaces > 0) {
                temp[index++] = ' ';
                nspaces--;
            }
            temp[index++] = line[i];
        }
    }

    temp[index++] = '\0';
    for (int i = 0; i < index; i++) {
        line[i] = temp[i];
    }
}
