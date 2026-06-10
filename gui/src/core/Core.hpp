/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#pragma once
#include <Font.hpp>
#include <raylib-cpp.hpp>
#include "../network/Socket.hpp"
#include "../server/Server.hpp"
#include "../Colors/Colors.hpp"
namespace zappy {

    class Core {
      public:
        int  Run(Socket& socket);
        void createWindow();
        void handleEvents();
        void displayBasicGui();
        void displayServerElements(const Server& server);
        void displayPlayers(const Server& server, int offsetX, int offsetY, int tileSizeClamped);
        void setupColors();
        void displayStats(const Server& server);

      private:
        bool            _isFullscreen    = true;
        bool            _isFpsDisplayed  = true;
        bool            _isCursorDefault = true;
        raylib::Image   _logo;
        raylib::Font    _font;
        raylib::Texture _cursor;
        raylib::Window  _window;
        Colors          _colors;
    };

}