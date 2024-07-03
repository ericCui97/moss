#include "compiler.h"
#include <stdio.h>
#include "scanner.h"

void compile(const char* source)
{
    init_scanner(source);
    int line = -1;
    for (;;) {
        Token token = scan_token();
        if (token.line != line) {
            printf("%4d ", token.line);
            line = token.line;
        } else {
            printf("   | ");
        }
        printf("%2d '%.*s' %s\n", token.type, token.length, token.start,
               tokentype_2_string(token.type));

        if (token.type == TOKEN_EOF)
            break;
    }
}