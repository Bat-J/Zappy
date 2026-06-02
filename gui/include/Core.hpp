/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#pragma once
#include <raylib-cpp.hpp>
#include "../include/Socket.hpp"
class Core {
  public:
    int  Run(Socket& socket);
    void createWindow();
    void handleEvents();
    void displayAll();

  private:
    bool           _isFullscreen    = true;
    bool           _isFpsDisplayed  = true;
    bool           _isCursorDefault = true;
    Image          logo             = ::LoadImage("assets/logo.png");
    raylib::Window _window;
};
