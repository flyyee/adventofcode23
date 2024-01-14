// gcc src_part2.c -Wall -Wextra -fsanitize={address,undefined} -g -o src && ./src
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>

void fatal(char *msg)
{
    puts(msg);
    exit(1);
}

// Checks if all characters in a string are the same
int strnall(const char *s, char c, int num)
{
    for (int i = 0; i < num; i++)
    {
        if (s[i] != c)
        {
            return 0;
        }
    }

    return 1;
}

// Checks if all characters in a string are the same (strided)
int strnall_strided(const char *s, char c, int num, int stride)
{
    for (int i = 0; i < num; i++)
    {
        if (s[i * stride] != c)
        {
            return 0;
        }
    }

    return 1;
}

typedef struct vector
{
    void *mem;
    unsigned long element_size;
    unsigned long length;
    unsigned long capacity;
} vector;

void vector_new(vector *v, unsigned long element_size, unsigned long capacity)
{
    v->element_size = element_size;
    v->length = 0;
    v->capacity = capacity;
    if ((v->mem = malloc(element_size * capacity)) == 0)
    {
        fatal("vector_new malloc");
    }
}

void vector_append(vector *v, void *p_element)
{
    if (v->length + 1 > v->capacity)
    {
        if (v->capacity == 0)
        {
            v->capacity = 1;
        }
        else
        {
            v->capacity *= 2;
        }
        if ((v->mem = realloc(v->mem, v->capacity * v->element_size)) == NULL)
        {
            fatal("vector_append realloc");
        }
    }
    memcpy(v->mem + v->length * v->element_size, p_element, v->element_size);
    v->length += 1;
}

void *vector_at(vector *v, unsigned long idx)
{
    if (idx >= v->length)
    {
        fatal("vector_at invalid idx");
    }
    return v->mem + idx * v->element_size;
}

void vector_clear(vector *v)
{
    memset(v->mem, '\0', v->capacity * v->element_size);
    free(v->mem);
    v->mem = NULL;
    v->element_size = 0;
    v->length = 0;
    v->capacity = 0;
}

typedef struct Galaxy
{
    unsigned long row;
    unsigned long col;
} Galaxy;

unsigned long diff(unsigned long a, unsigned long b)
{
    return (a > b) ? a - b : b - a;
}

int galaxy_distance(Galaxy *g1, Galaxy *g2)
{
    return diff(g1->row, g2->row) + diff(g1->col, g2->col);
}

int main()
{
    FILE *fp = fopen("testcase.txt", "r");
    if (!fp)
    {
        fatal("file read");
    }
    if (fseek(fp, 0L, SEEK_END) == -1)
    {
        fatal("fseek");
    }
    long file_size = 0;
    if ((file_size = ftell(fp)) == -1)
    {
        fatal("ftell");
    }
    rewind(fp);

    char *file_contents = malloc(file_size + 1);
    if (!file_contents)
    {
        fatal("malloc");
    }

    fread(file_contents, 1, file_size, fp);
    file_contents[file_size] = '\0';
    fclose(fp);

    const int width = strchr(file_contents, '\n') - file_contents;
    // each 'line' has width characters + 1 newline, except for the last line which has no newline
    const int height = (strlen(file_contents) + 1) / (width + 1);

    // 1. iterate over all rows to find empty rows
    vector empty_rows;
    vector_new(&empty_rows, sizeof(int), 0);
    for (int i = 0; i < height; i++)
    {
        if (strnall(&file_contents[i * (width + 1)], '.', width))
        {
            // this is an empty row
            vector_append(&empty_rows, &i);
        }
    }

    // 2. iterate over all columns to find empty columns
    vector empty_cols;
    vector_new(&empty_cols, sizeof(int), 0);
    for (int i = 0; i < width; i++)
    {
        if (strnall_strided(&file_contents[i], '.', height, width + 1))
        {
            // this is an empty row
            vector_append(&empty_cols, &i);
        }
    }

    // 3. Look for galaxies
    vector galaxies;
    vector_new(&galaxies, sizeof(Galaxy), 0);
    for (int i = 0; i < height; i++)
    {
        for (int j = 0; j < width; j++)
        {
            if (file_contents[i * (width + 1) + j] == '#')
            {
                // Galaxy
                Galaxy g;
                g.row = i;
                g.col = j;
                vector_append(&galaxies, &g);
            }
        }
    }

    // 4. Modify galaxy positions based on expansion
    unsigned long galaxy_row_number = -1;
    unsigned long empty_cols_prior = 0;
    unsigned long empty_rows_prior = 0;
    for (unsigned long i = 0; i < galaxies.length; i++)
    {
        Galaxy *p_galaxy = (Galaxy *)vector_at(&galaxies, i);
        if (p_galaxy->row != galaxy_row_number)
        {
            // we have started on a new row
            galaxy_row_number = p_galaxy->row;
            while (empty_rows_prior < empty_rows.length && *(int *)vector_at(&empty_rows, empty_rows_prior) < galaxy_row_number)
            {
                empty_rows_prior++;
            }

            empty_cols_prior = 0;
        }
        while (empty_cols_prior < empty_cols.length && *(int *)vector_at(&empty_cols, empty_cols_prior) < p_galaxy->col)
        {
            empty_cols_prior++;
        }
        p_galaxy->col += empty_cols_prior * (1000000 - 1);
        p_galaxy->row += empty_rows_prior * (1000000 - 1);
    }

    // 5. Compute the pairwise manhattan distances between galaxies
    unsigned long total_distance = 0;
    for (unsigned long i = 0; i < galaxies.length; i++)
    {
        for (unsigned long j = i + 1; j < galaxies.length; j++)
        {
            total_distance += galaxy_distance((Galaxy *)vector_at(&galaxies, i), (Galaxy *)vector_at(&galaxies, j));
        }
    }

    printf("Ans: %ld\n", total_distance);

    vector_clear(&galaxies);
    vector_clear(&empty_cols);
    vector_clear(&empty_rows);
    free(file_contents);
    return 0;
}