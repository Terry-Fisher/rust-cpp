#include <iostream>
#include <dlfcn.h>  // Для загрузки динамической библиотеки
#include <vector>
#include <map>



using namespace std;



// Определение типа функции, которую будем загружать
typedef void (*client_connect_license_fn)();
typedef void (*initial_license_key_for_connection_fn)(const uint8_t** ptr, size_t* len);
typedef void (*free_license_key_fn)(uint8_t* ptr, size_t len);


int init_0() {
    void* handle = dlopen("./liblib_0.so", RTLD_LAZY);
    if (!handle) {
        std::cerr << "Failed to load library: " << dlerror() << std::endl;
        return 1;
    }

    // Получаем указатель на функцию client_connect_license_0
    dlerror();  // Очищаем предыдущие ошибки
    client_connect_license_fn client_connect_license_0 = 
        (client_connect_license_fn) dlsym(handle, "client_connect_license_0");
    initial_license_key_for_connection_fn initial_license_key_for_connection0 = 
        (initial_license_key_for_connection_fn) dlsym(handle, "initial_license_key_for_connection0");
    // Освобождение памяти
    free_license_key_fn free_license_key = 
        (free_license_key_fn) dlsym(handle, "free_license_key");



    const char* dlsym_error = dlerror();
    if (dlsym_error) {
        std::cerr << "Failed to load function: " << dlsym_error << std::endl;
        dlclose(handle);
        return 1;
    }

    // Вызываем функцию несколько раз
    std::cout << "Calling client_connect_license_0 for the first time:" << std::endl;
    client_connect_license_0();

    std::cout << "Calling initial_license_key_for_connection0 for the second time:" << std::endl;
    // LicenseKey key = initial_license_key_for_connection0();
    // После использования данных освобождаем память

    const uint8_t* ptr;
    size_t len;

    // Вызов функции из Rust
    initial_license_key_for_connection0(&ptr, &len);
    // Работа с полученными данными
    std::vector<uint8_t> license(ptr, ptr + len);

    // Вывод данных для примера
    std::cout << "HASH : [";
    for (int i=0; i<len; i++) {
        if ( i == (len-1) ) {
            std::cout << (int)license[i] << "]";
        } else {
            std::cout << (int)license[i] << ", ";
        }
    }
    std::cout << std::endl;

    free_license_key((uint8_t*)ptr, len);

    // Закрываем библиотеку
    dlclose(handle);
    return 0;
}



int main() {
    
    cout << "Hello World!" << "\n";

    map<string, int> people = { {"John", 32}, {"Adele", 45}, {"Bo", 29} };

    cout << "John is: " << people["John"] << "\n";

    cout << "Adele is: " << people["Adele"] << "\n";

    init_0();

    int c = 2, d = 9;
    cout << c << " + " << d << " = " << c+d << "\n";

    return 0;
}
