/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#pragma once
#include <Color.hpp>
#include <raylib-cpp.hpp>
#include "../include/Socket.hpp"
#include "../include/Server.hpp"

class Core {
  public:
    int  Run(Socket& socket);
    void createWindow();
    void handleEvents();
    void displayBasicGui();
    void displayServerElements(const Server& server);
    void setupColors();

  private:
    bool                       _isFullscreen    = true;
    bool                       _isFpsDisplayed  = true;
    bool                       _isCursorDefault = true;
    raylib::Image              logo;
    raylib::Texture            cursor;
    raylib::Window             _window;
    std::vector<raylib::Color> _ressourceColors;
    raylib::Color              _TileColor;
    raylib::Color              _TileBorderColor;
    raylib::Color              _BackgroundColor;
    size_t                     _tileColorIndex       = 0;
    size_t                     _tileBorderColorIndex = 0;
    size_t                     _backgroundColorIndex = 0;
};
