#!/bin/bash

# Установка пути к библиотекам
# export LD_LIBRARY_PATH=$(pwd)/lib
# export LD_LIBRARY_PATH=$(pwd)/lib-0/target/release
export LD_LIBRARY_PATH=$(pwd)/lib-0/target/release:$LD_LIBRARY_PATH

# Запуск программы
echo "Running the program..."
./bin
