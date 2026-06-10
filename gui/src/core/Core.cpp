/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#include "Core.hpp"
#include "Circle.hpp"
#include <algorithm>
#include <chrono>
#include <future>
#include <string>
#include <ctime>

namespace zappy {
    void Core::setupColors() {
        _colors.setupColors();
    }

    void Core::createWindow() {
        _window.Init(1920, 1080, "Zappy GUI", 0, LOG_NONE);
        _window.SetTargetFPS(60);
        _window.SetFullscreen(_isFullscreen);
        _logo   = LoadImage("assets/images/logo.png");
        _cursor = LoadTexture("assets/images/cursor.png");
        _window.SetIcon(_logo);
        _font = raylib::Font("assets/font/akashi.ttf");
        _logo.Unload();
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
        _colors.handleEvents();
    }

    void Core::displayBasicGui() {
        if (_isFpsDisplayed == true) {
            _window.DrawFPS(20, 20);
        }
        if (_isCursorDefault == false) {
            _window.HideCursor();
            raylib::Vector2 pos = raylib::Mouse::GetPosition();
            _cursor.Draw(static_cast<int>(pos.x), static_cast<int>(pos.y), WHITE);
        } else {
            _window.ShowCursor();
        }
    }

    void Core::displayStats(const Server& server) {
        const int   x        = _window.GetWidth() / 2;
        const int   y        = 110;
        const int   fontSize = 15;
        const float spacing  = 2.0f;

        _font.DrawText("INFORMATIONS :", x, y, fontSize + 10, spacing, _colors.getTextColor());
        _font.DrawText("Players: " + std::to_string(server.players().getNbPlayers()), x, y + 40, fontSize, spacing, _colors.getTextColor());
        _font.DrawText("Deaths: " + std::to_string(server.players().getNbDeath()), x + 150, y + 40, fontSize, spacing, _colors.getTextColor());
        _font.DrawText("Frequency: " + std::to_string(server.world().getTimeReference()), x + 300, y + 40, fontSize, spacing, _colors.getTextColor());
        _font.DrawText("Map size: " + std::to_string(server.world().getMapSize().first) + "x" + std::to_string(server.world().getMapSize().second), x + 450, y + 40, fontSize,
                       spacing, _colors.getTextColor());
        _font.DrawText("TOTAL RESOURCES :", x, y + 80, fontSize + 10, spacing, _colors.getTextColor());
        _font.DrawText("Food: " + std::to_string(server.resources().getTotalFood()), x, y + 120, fontSize, spacing, _colors.getTextColor());
        _font.DrawText("Linemate: " + std::to_string(server.resources().getTotalLinemate()), x + 180, y + 120, fontSize, spacing, _colors.getTextColor());
        _font.DrawText("Deraumere: " + std::to_string(server.resources().getTotalDeraumere()), x + 370, y + 120, fontSize, spacing, _colors.getTextColor());
        _font.DrawText("Sibur: " + std::to_string(server.resources().getTotalSibur()), x + 570, y + 120, fontSize, spacing, _colors.getTextColor());
        _font.DrawText("Mendiane: " + std::to_string(server.resources().getTotalMendiane()), x + 730, y + 120, fontSize, spacing, _colors.getTextColor());
        _font.DrawText("Phiras: " + std::to_string(server.resources().getTotalPhiras()), x, y + 150, fontSize, spacing, _colors.getTextColor());
        _font.DrawText("Thystame: " + std::to_string(server.resources().getTotalThystame()), x + 160, y + 150, fontSize, spacing, _colors.getTextColor());
    }

    void Core::displayPlayers(const Server& server, int offsetX, int offsetY, int tileSizeClamped) {
        auto& players = server.players().getPlayer();
        int   id      = 1;
        for (const auto& player : players) {
            if (!player.getIsAlive()) {
                id++;
                continue;
            }
            const int x = player.getPos().first.first;
            const int y = player.getPos().first.second;
            if (x < 0 || y < 0)
                continue;

            const int      playerX = offsetX + x * tileSizeClamped + tileSizeClamped / 2;
            const int      playerY = offsetY + y * tileSizeClamped + tileSizeClamped / 2;

            raylib::Circle circle;
            circle.CircleDraw(playerX, playerY, std::max(4, tileSizeClamped / 6), _colors.getPlayerColor());
            _font.DrawText("P" + std::to_string(id), playerX - 6, playerY - 8, 12, 1.0f, _colors.getPlayerNumberColor());
            id++;
        }
    }

    void Core::displayServerElements(const Server& server) {
        const auto  mapSize         = server.world().getMapSize();
        const int   mapWidth        = std::max(1, (int)mapSize.first);
        const int   mapHeight       = std::max(1, (int)mapSize.second);
        const int   hudHeight       = 100;
        const int   padding         = 10;
        const int   availableWidth  = (_window.GetWidth() / 2) - 2 * padding;
        const int   tileSize        = std::min(availableWidth / mapWidth, (_window.GetHeight() - hudHeight - 2 * padding) / mapHeight);
        const int   tileSizeClamped = std::max(1, tileSize);
        const int   mapPixelWidth   = mapWidth * tileSizeClamped;
        const int   mapPixelHeight  = mapHeight * tileSizeClamped;
        const int   offsetX         = padding;
        const int   offsetY         = hudHeight + padding;

        const auto& tiles = server.world().getTiles();
        const Tile  fallbackTile{};
        for (int y = 0; y < mapHeight; ++y) {
            for (int x = 0; x < mapWidth; ++x) {
                const int        rectX     = offsetX + x * tileSizeClamped;
                const int        rectY     = offsetY + y * tileSizeClamped;
                const int        tileIndex = y * mapWidth + x;
                const int        textSize  = std::max(8, std::min(14, tileSizeClamped / 4));
                const Tile&      tile      = (tileIndex < static_cast<int>(tiles.size())) ? tiles[tileIndex] : fallbackTile;
                std::vector<int> resources = tile.getRessources();
                resources.resize(7, 0);
                raylib::Rectangle rect(rectX, rectY, tileSizeClamped - 1, tileSizeClamped - 1);
                rect.Draw(_colors.getTileColor());
                rect.DrawLines(_colors.getTileBorderColor());
                const std::vector<std::pair<int, int>> positions = {
                    {2, 2},
                    {tileSizeClamped / 2, 2},
                    {tileSizeClamped - 12, 2},
                    {2, tileSizeClamped / 2},
                    {tileSizeClamped / 2, tileSizeClamped / 2},
                    {tileSizeClamped - 12, tileSizeClamped / 2},
                    {tileSizeClamped / 2, tileSizeClamped - 12},
                };
                for (size_t i = 0; i < positions.size(); ++i) {
                    _font.DrawText(std::to_string(resources[i]), rectX + positions[i].first, rectY + positions[i].second, textSize, 1.0f, _colors.getTextColor());
                }
            }
        }
        displayPlayers(server, offsetX, offsetY, tileSizeClamped);
        _font.DrawText("ZAPFRITES", (_window.GetSize().GetX() / 2) - 3 * 50, 20, 50, 5, _colors.getTextColor());
        displayStats(server);
    }

    int Core::Run(Socket& socket) {
        setupColors();
        createWindow();
        Server server(socket);
        server.getInfo();
        server.updateResources();
        auto              lastUpdate = std::chrono::steady_clock::now();
        std::future<void> updateFuture;
        while (!_window.ShouldClose()) {
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
            _window.BeginDrawing();
            _window.ClearBackground(_colors.getBackgroundColor());
            displayServerElements(server);
            displayBasicGui();
            handleEvents();
            _window.EndDrawing();
        }
        _cursor.Unload();
        _font.Unload();
        _window.Close();
        return 0;
    }
}