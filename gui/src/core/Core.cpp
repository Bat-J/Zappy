/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#include "Core.hpp"
#include "Circle.hpp"
#include <Keyboard.hpp>
#include <algorithm>
#include <chrono>
#include <future>
#include <raylib.h>
#include <string>
#include <ctime>

namespace zappy {
    void Core::setupColors() {
        _ressourceColors                        = {RED, GREEN, BLUE, YELLOW, PURPLE, ORANGE, PINK, BROWN, GRAY, LIGHTGRAY, MAROON, LIME, SKYBLUE, VIOLET, BEIGE, DARKBROWN, GOLD};
        std::vector<raylib::Color> secondColors = {DARKGREEN, DARKBLUE, DARKPURPLE, DARKBROWN, BLACK, WHITE};
        _ressourceColors.insert(_ressourceColors.end(), secondColors.begin(), secondColors.end());
        _TileColor            = GRAY;
        _tileColorIndex       = 8;
        _TileBorderColor      = BLACK;
        _tileBorderColorIndex = 21;
        _TextColor            = WHITE;
        _TextColorIndex       = 22;
        _BackgroundColor      = BLACK;
        _backgroundColorIndex = 21;
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
        if (raylib::Keyboard::IsKeyPressed(KEY_ONE)) {
            _tileColorIndex = (_tileColorIndex + 1) % _ressourceColors.size();
            while (_TextColorIndex == _tileColorIndex || _backgroundColorIndex == _tileColorIndex || _tileColorIndex == 0)
                _tileColorIndex = (_tileColorIndex + 1) % _ressourceColors.size();
            _TileColor = _ressourceColors[_tileColorIndex];
        }
        if (raylib::Keyboard::IsKeyPressed(KEY_TWO)) {
            _tileBorderColorIndex = (_tileBorderColorIndex + 1) % _ressourceColors.size();
            _TileBorderColor      = _ressourceColors[_tileBorderColorIndex];
        }
        if (raylib::Keyboard::IsKeyPressed(KEY_THREE)) {
            _backgroundColorIndex = (_backgroundColorIndex + 1) % _ressourceColors.size();
            while (_backgroundColorIndex == _tileColorIndex || _backgroundColorIndex == _TextColorIndex)
                _backgroundColorIndex = (_backgroundColorIndex + 1) % _ressourceColors.size();
            _BackgroundColor = _ressourceColors[_backgroundColorIndex];
        }
        if (raylib::Keyboard::IsKeyPressed(KEY_FOUR)) {
            _TextColorIndex = (_TextColorIndex + 1) % _ressourceColors.size();
            while (_TextColorIndex == _backgroundColorIndex || _TextColorIndex == _tileColorIndex)
                _TextColorIndex = (_TextColorIndex + 1) % _ressourceColors.size();
            _TextColor = _ressourceColors[_TextColorIndex];
        }
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
        const int   x        = _window.GetWidth() / 1.80;
        const int   y        = _window.GetHeight() / 1.80;
        const int   fontSize = 22;
        const float spacing  = 2.0f;

        _font.DrawText("Players: " + std::to_string(server.players().getNbPlayers()), x, y, fontSize, spacing, _TextColor);
        _font.DrawText("Deaths: " + std::to_string(server.players().getNbDeath()), x, y + 30, fontSize, spacing, _TextColor);
        _font.DrawText("Time ref.: " + std::to_string(server.world().getTimeReference()), x, y + 60, fontSize, spacing, _TextColor);
        _font.DrawText("Map size: " + std::to_string(server.world().getMapSize().first) + "x" + std::to_string(server.world().getMapSize().second), x, y + 90, fontSize, spacing,
                       _TextColor);
        _font.DrawText("Food: " + std::to_string(server.resources().getTotalFood()), x, y + 120, fontSize, spacing, _TextColor);
        _font.DrawText("Linemate: " + std::to_string(server.resources().getTotalLinemate()), x + 180, y + 120, fontSize, spacing, _TextColor);
        _font.DrawText("Deraumere: " + std::to_string(server.resources().getTotalDeraumere()), x + 360, y + 120, fontSize, spacing, _TextColor);
        _font.DrawText("Sibur: " + std::to_string(server.resources().getTotalSibur()), x, y + 150, fontSize, spacing, _TextColor);
        _font.DrawText("Mendiane: " + std::to_string(server.resources().getTotalMendiane()), x + 180, y + 150, fontSize, spacing, _TextColor);
        _font.DrawText("Phiras: " + std::to_string(server.resources().getTotalPhiras()), x + 360, y + 150, fontSize, spacing, _TextColor);
        _font.DrawText("Thystame: " + std::to_string(server.resources().getTotalThystame()), x, y + 180, fontSize, spacing, _TextColor);
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
            circle.CircleDraw(playerX, playerY, std::max(4, tileSizeClamped / 6), RED);
            _font.DrawText("P" + std::to_string(id), playerX - 6, playerY - 8, 12, 1.0f, BLACK);
            id++;
        }
    }

    void Core::displayServerElements(const Server& server) {
        const auto  mapSize         = server.world().getMapSize();
        const int   mapWidth        = std::max(1, (int)mapSize.first);
        const int   mapHeight       = std::max(1, (int)mapSize.second);
        const int   hudHeight       = 100;
        const int   padding         = 10;
        const int   tileSize        = std::min((_window.GetWidth() - 2 * padding) / mapWidth, (_window.GetHeight() - hudHeight - 2 * padding) / mapHeight);
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
                rect.Draw(_TileColor);
                rect.DrawLines(_TileBorderColor);
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
                    _font.DrawText(std::to_string(resources[i]), rectX + positions[i].first, rectY + positions[i].second, textSize, 1.0f, _TextColor);
                }
            }
        }
        displayPlayers(server, offsetX, offsetY, tileSizeClamped);
        _font.DrawText("ZAPFRITES", (_window.GetSize().GetX() / 2) - 3 * 50, 20, 50, 5, _TextColor);
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
            _window.ClearBackground(_BackgroundColor);
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