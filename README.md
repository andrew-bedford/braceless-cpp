# braceless-cpp
An experimental C/C++ preprocessor to show what C/C++ could look like if it used Python-styled indendation to denote scope instead of braces.

## Usage
Prefix calls to `clang++` with `braceless` like so:
```
braceless clang++ -c test.bpp
```
It will detect the file being compiled (e.g., `test.bpp`), create a hidden copy of it named `.test.cpp` in the same folder as the original source file, and then insert braces based on the indentation before invoking `clang++` on `.test.cpp`. If a file does not have a `.bpp` extension, it is forwarded without any modifications to the compiler.

## Example
```
#include <iostream>

int main()
    std::cout << "Hello!" << std::endl;
    for (int i = 0; i < 10; ++i)
        std::cout << i;
        std::cout << std::endl;
```
Would be converted to:
```
#include <iostream>

int main() {
    std::cout << "Hello!" << std::endl;
    for (int i = 0; i < 10; ++i) {
        std::cout << i;
        std::cout << std::endl;
    }
}
```

## Building
```
cargo build
cargo test
```

## Why?
Just a fun experiment to learn about Rust. I don't necessarily think that C/C++ would be better without braces, although it does look nice in some cases.

But also, while I like C/C++, there's many things that I'd like to be different in the language such as:
 - using `var` instead of `auto` for variable declarations,
 - using postfix type notations `i:int` like Rust and Pascal
 - default to smart pointers
 - default to `const` unless `mut` is used
 - eliminate undefined behaviours, maybe by enabling lightweight sanitizers by default
 - a more complete std library
 - ...

So, what if we introduced more "transformers" like `braceless` that could be chained together so that one could use a variant that they like?