/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#include "Core.hpp"
#include <Rectangle.hpp>
#include <Window.hpp>
#include <chrono>
#include <future>
#include <raylib.h>

namespace zappy {

    void Core::handleEvents() {
        raylib::Window window = _renderer.getWindow();
        Colors         colors = _renderer.getColors();

        if (raylib::Keyboard::IsKeyPressed(KEY_F3))
            _isFpsDisplayed = !_isFpsDisplayed;
        if (raylib::Keyboard::IsKeyPressed(KEY_F11)) {
            _isFullscreen = !_isFullscreen;
            window.SetFullscreen(_isFullscreen);
        }
        if (raylib::Keyboard::IsKeyPressed(KEY_F2))
            _isCursorDefault = !_isCursorDefault;
        colors.handleEvents();
    }

    int Core::Run(Socket& socket) {
        _renderer.setupColors();
        _renderer.createWindow(_isFullscreen);
        Server server(socket);
        server.getInfo();
        server.updateResources();
        auto              lastUpdate = std::chrono::steady_clock::now();
        std::future<void> updateFuture;
        raylib::Window    window = _renderer.getWindow();
        while (!window.ShouldClose()) {
            const auto now        = std::chrono::steady_clock::now();
            const auto elapsedMs  = std::chrono::duration_cast<std::chrono::milliseconds>(now - lastUpdate).count();
            const auto t          = server.world().getTimeReference();
            const auto intervalMs = (t > 0) ? static_cast<long long>(1000.0 / t) : 1000LL;
            if (elapsedMs >= intervalMs && !updateFuture.valid()) {
                updateFuture    = std::async(std::launch::async, [&server]() {
                    server.updateTiles();
                    server.updateResources();
                    server.handleServerUpdate();
                });
                auto lastUpdate = std::chrono::steady_clock::now() - std::chrono::milliseconds(10000);
            }
            if (updateFuture.valid() && updateFuture.wait_for(std::chrono::milliseconds(0)) == std::future_status::ready) {
                updateFuture.get();
            }
            _renderer.displayAll(server, _isFpsDisplayed, _isCursorDefault);
        }
        _renderer.close();
        return 0;
    }
}