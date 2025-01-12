# Rust implementation for different C functions and libraries 

The printf function is designed to replicate the behavior of the standard C library's printf function. It handles various format specifiers to print different types of data. The project involves practicing the use of variadic functions, specifically dealing with a variable number of argument


## Features

The printf function should handle the following format specifiers:


| Argument             | What does it do                                                 |
| ----------------- | ------------------------------------------------------------------ |
| %c | Prints a single character. |
| %s | Prints a string. |
| %d | Prints a decimal (base 10) number. |
| %i | Prints an integer in base 10. |

The scanf function will handle taking the input from user using the specifiers mentioned above


Also, the following functions were implemented: 

| Function             | What does it do                                                 |
| ----------------- | ------------------------------------------------------------------ |
| abs | Returns the absolute value of a number |
| c_func | Implementations of different C functions like atoi, itoa, is_alnum etc. |


Math function:

| Function             | What does it do                                                 |
| ----------------- | ------------------------------------------------------------------ |
| abs | Returns the absolute value of a number |
| sin | This function takes angle (in radian) as an argument and returns its sine value that could be verified using sine curve |
| cos | This function takes angle (in radians) as an argument and returns its cosine value that could be verified using cosine curve. |
| tan | This function takes angle (in radians) as an argument and returns its tangent value. This could also be verified using Trigonometry as Tan(x) = Sin(x)/Cos(x). |


The List folder contains the implementation for doubly linked list


| File              | What does it do                                                    |
| ----------------- | ------------------------------------------------------------------ |
| deque.rs | Contains the Deque struct, which implements the doubly linked list API. |
| node.rs| Defines the Node struct, representing each node in the list. |
| mem | Manages memory for nodes using a custom allocator. |
| lib.rs | Handles the integration of the List module into the project. |
| mod.rs | Exposes the public interface of the List module and ties all components. |
| list.rs | The actual implementation of List library |

