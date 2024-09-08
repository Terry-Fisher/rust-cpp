#!/bin/bash


# Компиляция библиотеки
echo "Compiling the library..."
cd ./lib-0
cargo build --release --lib
cd ..

# Копируем библиотеку в рабочую директорию
cp ./lib-0/target/release/liblib_0.so .

# Компиляция основной программы с указанием пути к библиотеке
echo "Compiling the main program..."
g++ -std=c++23 -o bin bin.cpp -lpthread -ldl

echo "Build completed!"
