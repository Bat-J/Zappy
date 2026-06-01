/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#include "../include/Core.hpp"

int Core::Run() {
    _window = raylib::Window(800, 600, "Zappy GUI");
    while (!_window.ShouldClose()) {
        _window.BeginDrawing();
    }
    return 0;
}