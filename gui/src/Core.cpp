/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#include "../include/Core.hpp"
#include <iostream>
#include <raylib.h>

int Core::Run(Socket& socket) {
    raylib::Window window(1920, 1080, "Zappy GUI");
    raylib::Color  textColor = raylib::Color::LightGray();
    this->_window            = window;
    _window.SetTargetFPS(60);
    _window.SetFullscreen(_isFullscreen);
    while (!_window.ShouldClose()) {
        _window.BeginDrawing();
        _window.ClearBackground(BLACK);
        textColor.DrawText("exit", 50, 50, 20);
        if (raylib::Keyboard::IsKeyPressed(KEY_F11)) {
            _isFullscreen = !_isFullscreen;
            _window.SetFullscreen(_isFullscreen);
        }
        _window.EndDrawing();
    }

    _window.Close();
    socket.writeToSocket("Hello, server!");
    std::string message = socket.readFromSocket();
    std::cout << "Message from server: " << message << std::endl;
    socket.closeSocket();
    return 0;
}