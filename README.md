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
