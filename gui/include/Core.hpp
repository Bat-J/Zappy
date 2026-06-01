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
    int Run(Socket& socket);

  private:
    raylib::Window _window;
    bool           _isFullscreen = true;
};
