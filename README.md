# Rust printf implementation

The printf function is designed to replicate the behavior of the standard C library's printf function. It handles various format specifiers to print different types of data. The project involves practicing the use of variadic functions, specifically dealing with a variable number of argument


## Features

The ft_printf function should handle the following format specifiers:


| Argument             | What does it do                                                 |
| ----------------- | ------------------------------------------------------------------ |
| %c | Prints a single character. |
| %s | Prints a string. |
| %p | Prints a void pointer in hexadecimal format. |
| %d | Prints a decimal (base 10) number. |
| %i | Prints an integer in base 10. |
| %u | Prints an unsigned decimal (base 10) number. |
| %x | Prints a number in hexadecimal (base 16) lowercase format. |
| %X |  Prints a number in hexadecimal (base 16) uppercase format. |
| %% |  Prints a percent sign. |


Also, the following functions were implemented: 

| Function             | What does it do                                                 |
| ----------------- | ------------------------------------------------------------------ |
| abs | Returns the absolute value of a number |
| aliquot_sum | Calculates the sum of the proper divisors of a number. |
| amicable_nums | Returns vec of amicable pairs below N. |
| c_func | Implementations of different C functions like atoi, itoa, is_alnum etc. |

Math function:

| Function             | What does it do                                                 |
| ----------------- | ------------------------------------------------------------------ |
| abs | Returns the absolute value of a number |
| sin | This function takes angle (in radian) as an argument and returns its sine value that could be verified using sine curve |
| cos | This function takes angle (in radians) as an argument and returns its cosine value that could be verified using cosine curve. |
| tan | This function takes angle (in radians) as an argument and returns its tangent value. This could also be verified using Trigonometry as Tan(x) = Sin(x)/Cos(x). |
