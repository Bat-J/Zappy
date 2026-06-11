/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Renderer
*/

#pragma once

#include "../server/Server.hpp"
#include "Colors.hpp"
#include <raylib-cpp.hpp>

namespace zappy {

    class Renderer {
      public:
        void                                        createWindow(bool _isFullscreen);
        void                                        displayTiles(const Server& server);
        void                                        drawTiles();
        void                                        updateTiles(const Server& server);
        void                                        displayBasicGui(bool isFpsDisplayed, bool isCursorDefault);
        void                                        displayServerElements(const Server& server);
        void                                        displayPlayers(const Server& server, int offsetX, int offsetY, int tileSizeClamped);
        void                                        setupColors();
        void                                        displayStats(const Server& server);
        void                                        displaySelectedTile(const Server& server);
        void                                        displayTotalResources(const Server& server);
        void                                        displayAll(const Server& server, bool isFpsDisplayed, bool isCursorDefault);
        void                                        close(void);
        raylib::Window&                             getWindow();
        Colors&                                     getColors();
        std::vector<std::vector<raylib::Rectangle>> getDrawnTiles();
        void                                        setSelectedTile(std::pair<int, int>& pair);

      private:
        raylib::Image                               _logo;
        raylib::Font                                _font;
        raylib::Texture                             _cursor;
        raylib::Window                              _window;
        std::vector<std::vector<raylib::Rectangle>> _drawnTiles;
        std::pair<int, int>                         _selectedTile;
        Colors                                      _colors;
        int                                         _textBeginPosX;
        const int                                   _textBeginPosY = 110;
        const int                                   _fontSize      = 15;
        const float                                 _spacing       = 2.0f;
    };
}