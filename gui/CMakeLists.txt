add_executable(zappy_gui)

target_sources(zappy_gui
    PRIVATE
        src/Socket.cpp
        src/Gui.cpp
        src/Error.cpp
)

target_include_directories(zappy_gui
    PRIVATE
        include
)

cmake_minimum_required(VERSION 3.16)
project(ZapfritesGui)