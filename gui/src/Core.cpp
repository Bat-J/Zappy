/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#include "../include/Core.hpp"
#include <iostream>

void Core::createWindow() {
    _window.Init(1920, 1080, "Zappy GUI");
    _window.SetTargetFPS(60);
    _window.SetFullscreen(_isFullscreen);
    logo   = LoadImage("assets/logo.png");
    cursor = LoadTexture("assets/cursor.png");
    _window.SetIcon(logo);
    logo.Unload();
}

void Core::handleEvents() {
    if (raylib::Keyboard::IsKeyPressed(KEY_F3))
        _isFpsDisplayed = !_isFpsDisplayed;
    if (raylib::Keyboard::IsKeyPressed(KEY_F11)) {
        _isFullscreen = !_isFullscreen;
        _window.SetFullscreen(_isFullscreen);
    }
    if (raylib::Keyboard::IsKeyPressed(KEY_F2))
        _isCursorDefault = !_isCursorDefault;
}

void Core::displayAll() {
    if (_isFpsDisplayed == true) {
        _window.DrawFPS(20, 20);
    }
    if (_isCursorDefault == false) {
        _window.HideCursor();
        raylib::Vector2 pos = raylib::Mouse::GetPosition();
        cursor.Draw(static_cast<int>(pos.x), static_cast<int>(pos.y), WHITE);
    } else {
        _window.ShowCursor();
    }
}

int Core::Run(Socket& socket) {
    createWindow();
    while (!_window.ShouldClose()) {
        _window.BeginDrawing();
        _window.ClearBackground(BLACK);
        displayAll();
        handleEvents();
        _window.EndDrawing();
    }
    // unload GPU resources while context is valid
    cursor.Unload();
    _window.Close();
    socket.writeToSocket("Hello, server!");
    std::string message = socket.readFromSocket();
    std::cout << "Message from server: " << message << std::endl;
    socket.closeSocket();
    return 0;
}