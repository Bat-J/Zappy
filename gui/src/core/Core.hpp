/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#pragma once
#include <Font.hpp>
#include <Rectangle.hpp>
#include <raylib-cpp.hpp>
#include "../network/Socket.hpp"
#include "../Renderer/Renderer.hpp"

namespace zappy {

    class Core {
      public:
        int  Run(Socket& socket);
        void handleEvents();

      private:
        bool     _isFullscreen    = true;
        bool     _isFpsDisplayed  = true;
        bool     _isCursorDefault = true;
        Renderer _renderer;
    };

}