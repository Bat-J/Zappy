cmake_minimum_required(VERSION 3.16)
project(ZapfritesGui)


find_package(raylib REQUIRED)

add_executable(zappy_gui 
    src/Socket.cpp
    src/Gui.cpp
    src/Error.cpp
    src/Core.cpp
)

target_include_directories(zappy_gui PRIVATE include)

target_link_libraries(zappy_gui PRIVATE raylib)