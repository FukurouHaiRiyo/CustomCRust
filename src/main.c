#include <stdarg.h>
#include <stdio.h>

// custon scanf() 
int myscanf(const char* str, ...) 
{ 
    char token[100]; 
    int k = 0; 
  
    // initializing list pointer 
    va_list ptr; 
    va_start(ptr, str); 
  
    // parsing the formatted string 
    for (int i = 0; str[i] != '\0'; i++) { 
  
        // copying str to token 
        token[k++] = str[i]; 
  
        // When a format specifier of null character is 
        // found 
        if (str[i + 1] == '%' || str[i + 1] == '\0') { 
            token[k] = '\0'; 
            k = 0; 
  
            // processing token 
            char ch1 = token[1]; 
  
            // for integers 
            if (ch1 == 'i' || ch1 == 'd' || ch1 == 'u') { 
                fscanf(stdin, "%i", va_arg(ptr, int*)); 
            } 
  
            // for short ubt 
            else if (ch1 == 'h') { 
                fscanf(stdin, "%hi", va_arg(ptr, short*)); 
            } 
  
            // for characters 
            else if (ch1 == 'c') { 
                char c; 
  
                // using this loop to ignore some chars 
                while ((c = fgetc(stdin)) == '\n'
                       || c == ' ' || c == EOF) { 
                } 
                *va_arg(ptr, char*) = c; 
            } 
            // for float 
            else if (ch1 == 'f') { 
                fscanf(stdin, "%f", va_arg(ptr, float*)); 
            } 
            else if (ch1 == 'l') { 
                char ch2 = token[2]; 
  
                // for long int 
                if (ch2 == 'u' || ch2 == 'd'
                    || ch2 == 'i') { 
                    fscanf(stdin, "%li", 
                           va_arg(ptr, long*)); 
                } 
  
                // for double 
                else if (ch2 == 'f') { 
                    fscanf(stdin, "%lf", 
                           va_arg(ptr, double*)); 
                } 
            } 
            else if (ch1 == 'L') { 
                char ch2 = token[2]; 
  
                // for long int 
                if (ch2 == 'u' || ch2 == 'd'
                    || ch2 == 'i') { 
                    fscanf(stdin, "%Li", 
                           va_arg(ptr, long long*)); 
                } 
  
                // for long double 
                else if (ch2 == 'f') { 
                    fscanf(stdin, "%Lf", 
                           va_arg(ptr, long double*)); 
                } 
            } 
  
            // for string 
            else if (ch1 == 's') { 
                fscanf(stdin, "%s", va_arg(ptr, char*)); 
            } 
        } 
    } 
  
    // closing va_list 
    va_end(ptr); 
    return 0; 
} 

int main(){
    int a;
    myscanf("%d", &a);
    printf("%d\n", a);
}