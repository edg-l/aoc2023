#include <stdio.h>

/*
one
two
three
four
five
six
seven
eight
nine
zero
1
2
3
4
5
6
7
8
9
0
*/

int match(const char *s) {
    int a = -1, b, cs = 0;
    while(cs != -1) {
        retry:
        switch(cs) {
        found:
            if(a == -1)
                a = b;
        case 0:
            switch(*s) {
            default: cs = 0;
            break; case 'o': cs = 1;
            break; case 't': cs = 4;
            break; case 'f': cs = 11;
            break; case 's': cs = 18;
            break; case 'e': cs = 25;
            break; case 'n': cs = 30;
            break; case 'z': cs = 34;
            break;
            case '0':
            case '1':
            case '2':
            case '3':
            case '4':
            case '5':
            case '6':
            case '7':
            case '8':
            case '9':
                b = *s - '0';
                if(a == -1)
                    a = b;
                break;
            }
        break; case 1:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'n': cs = 2;
            }
        break; case 2:
            switch(*s) {
                default: cs = 30; goto retry;
                break; case 'e':
                    b = 1;
                    goto found;
            }
        break; case 4:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'w': cs = 5;
                break; case 'h': cs = 7;
            }
        break; case 5:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'o':
                    b = 2;
                    goto found;
            }
        break; case 7:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'r': cs = 8;
            }
        break; case 8:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'e': cs = 9;
            }
        break; case 9:
            switch(*s) {
                default: cs = 25; goto retry;
                break; case 'e':
                    b = 3;
                    goto found;
            }
        break; case 11:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'o': cs = 12;
                break; case 'i': cs = 15;
            }
        break; case 12:
            switch(*s) {
                default: cs = 1; goto retry;
                break; case 'u': cs = 13;
            }
        break; case 13:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'r':
                    b = 4;
                    goto found;
            }
        break; case 15:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'v': cs = 16;
            }
        break; case 16:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'e':
                    b = 5;
                    goto found;
            }
        break; case 18:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'i': cs = 19;
                break; case 'e': cs = 21;
            }
        break; case 19:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'x':
                    b = 6;
                    goto found;
            }
        break; case 21:
            switch(*s) {
                default: cs = 25; goto retry;
                break; case 'v': cs = 22;
            }
        break; case 22:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'e': cs = 23;
            }
        break; case 23:
            switch(*s) {
                default: cs = 25; goto retry;
                break; case 'n':
                    b = 7;
                    goto found;
            }
        break; case 25:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'i': cs = 26;
            }
        break; case 26:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'g': cs = 27;
            }
        break; case 27:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'h': cs = 28;
            }
        break; case 28:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 't':
                    b = 8;
                    goto found;
            }
        break; case 30:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'i': cs = 31;
            }
        break; case 31:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'n': cs = 32;
            }
        break; case 32:
            switch(*s) {
                default: cs = 30; goto retry;
                break; case 'e':
                    b = 9;
                    goto found;
            }
        break; case 34:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'e': cs = 35;
            }
        break; case 35:
            switch(*s) {
                default: cs = 25; goto retry;
                break; case 'r': cs = 36;
            }
        break; case 36:
            switch(*s) {
                default: cs = 0; goto retry;
                break; case 'o':
                    b = 0;
                    goto found;
            }
        }

        if(!*++s)
            cs = -1;
    }

    return 10 * a + b;
}

void get1(FILE *f, char *buf, size_t bufsz)
{
    int c;
    while((c = getc(f)) != EOF && c != '\n' && bufsz > 0)
        *buf++ = c;
    *buf = '\0';
}

int main(void)
{
    FILE *f = fopen("input.txt", "r");
    int sum = 0;
    for(;;) {
        char buf[64];
        get1(f, buf, sizeof buf);
        if(feof(f))
            break;

        int m = match(buf);
        sum += m;
    }

    printf("%d\n", sum);
    return 0;
}
