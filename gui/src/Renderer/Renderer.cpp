/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Renderer
*/

#include "Renderer.hpp"
#include "Circle.hpp"
#include <chrono>
#include <string>
#include <vector>
#include <algorithm>

namespace zappy {

    void Renderer::setupColors() {
        _colors.setupColors();
    }

    void Renderer::createWindow(bool _isFullscreen) {
        _window.Init(1920, 1080, "Zappy GUI", 0, LOG_NONE);
        _window.SetTargetFPS(60);
        _window.SetFullscreen(_isFullscreen);
        _logo   = LoadImage("assets/images/logo.png");
        _cursor = LoadTexture("assets/images/cursor.png");
        _window.SetIcon(_logo);
        _font = raylib::Font("assets/font/akashi.ttf");
        _logo.Unload();
        _textBeginPosX = _window.GetWidth() / 2;
    }

    void Renderer::displayBasicGui(bool isFpsDisplayed, bool isCursorDefault) {
        if (isFpsDisplayed == true) {
            _window.DrawFPS(20, 20);
        }
        if (isCursorDefault == false) {
            _window.HideCursor();
            raylib::Vector2 pos = raylib::Mouse::GetPosition();
            _cursor.Draw(static_cast<int>(pos.x), static_cast<int>(pos.y), WHITE);
        } else {
            _window.ShowCursor();
        }
    }

    void Renderer::displayStats(const Server& server) {
        const int x = _textBeginPosX;
        const int y = _textBeginPosY;

        _font.DrawText("INFORMATIONS :", x, y, _fontSize + 10, _spacing, _colors.getTextColor());
        _font.DrawText("Players: " + std::to_string(server.players().getNbPlayers()), x, y + 40, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Deaths: " + std::to_string(server.players().getNbDeath()), x + 150, y + 40, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Frequency: " + std::to_string(server.world().getTimeReference()), x + 300, y + 40, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Map size: " + std::to_string(server.world().getMapSize().first) + "x" + std::to_string(server.world().getMapSize().second), x + 450, y + 40, _fontSize,
                       _spacing, _colors.getTextColor());
    }

    void Renderer::displayTotalResources(const Server& server) {
        const int x = _textBeginPosX;
        const int y = _textBeginPosY;

        _font.DrawText("TOTAL RESOURCES :", x, y + 80, _fontSize + 10, _spacing, _colors.getTextColor());
        _font.DrawText("Food: " + std::to_string(server.resources().getTotalFood()), x, y + 120, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Linemate: " + std::to_string(server.resources().getTotalLinemate()), x + 180, y + 120, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Deraumere: " + std::to_string(server.resources().getTotalDeraumere()), x + 370, y + 120, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Sibur: " + std::to_string(server.resources().getTotalSibur()), x + 570, y + 120, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Mendiane: " + std::to_string(server.resources().getTotalMendiane()), x + 730, y + 120, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Phiras: " + std::to_string(server.resources().getTotalPhiras()), x, y + 150, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Thystame: " + std::to_string(server.resources().getTotalThystame()), x + 160, y + 150, _fontSize, _spacing, _colors.getTextColor());
    }

    void Renderer::displaySelectedTile(const Server& server) {
        const int  x    = _textBeginPosX;
        const int  y    = _textBeginPosY;
        const auto tile = server.world().catchTile(_selectedTile.first, _selectedTile.second);
        if (!tile)
            return;
        _font.DrawText("SELECTED TILE : (x = " + std::to_string(_selectedTile.first) + " y = " + std::to_string(_selectedTile.second) + ") :", x, y + 200, _fontSize + 10, _spacing,
                       _colors.getTextColor());
        _font.DrawText("Food: " + std::to_string(tile->getRessources()[0]), x + 425, y + 205, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Linemate: " + std::to_string(tile->getRessources()[1]), x + 525, y + 205, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Deraumere: " + std::to_string(tile->getRessources()[2]), x + 650, y + 205, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Sibur: " + std::to_string(tile->getRessources()[3]), x + 800, y + 205, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Mendiane: " + std::to_string(tile->getRessources()[4]), x, y + 230, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Phiras: " + std::to_string(tile->getRessources()[5]), x + 150, y + 230, _fontSize, _spacing, _colors.getTextColor());
        _font.DrawText("Thystame: " + std::to_string(tile->getRessources()[5]), x + 300, y + 230, _fontSize, _spacing, _colors.getTextColor());
    }

    void Renderer::displaySelectedPlayer(const Server& server) {
        int   x       = 1000;
        int   y       = 200;
        auto& players = server.players().getPlayer();
        if (!_selectedPlayerId) {
            _font.DrawText("No Player selected or found.", x + 200, 400, 20, _spacing, _colors.getTextColor());
            return;
        }
        auto p = std::find_if(players.begin(), players.end(), [&](const Player& p) { return p.getId() == *_selectedPlayerId; });
        _font.DrawText("SELECTED Player : " + std::to_string(_selectedPlayerId.value() + 1), x + 300, y + 200, _fontSize + 10, _spacing, _colors.getTextColor());
        if (!p->getIsAlive()) {
            _font.DrawText("Status : DEAD", x + 300, y + 250, _fontSize, _spacing, _colors.getTextColor());
        } else {
            _font.DrawText("Status : ALIVE", x + 300, y + 250, _fontSize, _spacing, _colors.getTextColor());
            _font.DrawText("Food : " + std::to_string(p->getFood()), x + 300, y + 280, _fontSize, _spacing, _colors.getTextColor());
            _font.DrawText("Linemate : " + std::to_string(p->getLinemate()), x + 300, y + 310, _fontSize, _spacing, _colors.getTextColor());
            _font.DrawText("Deraumere : " + std::to_string(p->getDeraumere()), x + 300, y + 340, _fontSize, _spacing, _colors.getTextColor());
            _font.DrawText("Sibur : " + std::to_string(p->getSibur()), x + 300, y + 370, _fontSize, _spacing, _colors.getTextColor());
            _font.DrawText("Mendiane : " + std::to_string(p->getMendiane()), x + 300, y + 400, _fontSize, _spacing, _colors.getTextColor());
            _font.DrawText("Phiras : " + std::to_string(p->getPhiras()), x + 300, y + 430, _fontSize, _spacing, _colors.getTextColor());
            _font.DrawText("Thystame : " + std::to_string(p->getThystame()), x + 300, y + 460, _fontSize, _spacing, _colors.getTextColor());
        }
    }

    void Renderer::displayPlayers(Server& server, int offsetX, int offsetY, int tileSizeClamped) {
        auto& players = server.players().getPlayer();
        int   id      = 1;
        for (auto& player : players) {
            if (player.getId() < 0) {
                player.setId();
            }
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

    void Renderer::updateTiles(const Server& server) {
        const std::pair<size_t, size_t> mapSize         = server.world().getMapSize();
        const int                       mapWidth        = mapSize.first;
        const int                       mapHeight       = mapSize.second;
        const int                       hudHeight       = 100;
        const int                       padding         = 10;
        const int                       availableWidth  = (_window.GetWidth() / 2) - 2 * 10;
        const int                       tileSize        = std::min(availableWidth / mapWidth, (_window.GetHeight() - hudHeight - 2 * padding) / mapHeight);
        const int                       tileSizeClamped = tileSize;
        const int                       offsetX         = 10;
        const int                       offsetY         = 100 + padding;
        const std::vector<Tile>         tiles           = server.world().getTiles();
        const Tile                      fallbackTile{};

        _drawnTiles.clear();
        _tileResources.clear();
        for (int y = 0; y < mapHeight; ++y) {
            std::vector<raylib::Rectangle> line;
            std::vector<std::vector<int>>  resLine;
            for (int x = 0; x < mapWidth; ++x) {
                const int        rectX     = offsetX + x * tileSizeClamped;
                const int        rectY     = offsetY + y * tileSizeClamped;
                const int        tileIndex = y * mapWidth + x;
                const Tile&      tile      = (tileIndex < static_cast<int>(tiles.size())) ? tiles[tileIndex] : fallbackTile;
                std::vector<int> resources = tile.getRessources();
                resources.resize(7, 0);
                line.emplace_back(rectX, rectY, tileSizeClamped - 1, tileSizeClamped - 1);
                resLine.push_back(resources);
            }
            _drawnTiles.push_back(line);
            _tileResources.push_back(resLine);
        }
    }

    void Renderer::drawTiles() {
        static const ::Color resourceColors[7] = {RED, GREEN, BLUE, YELLOW, PURPLE, ORANGE, PINK};
        // dot layout: 2 columns, 4 rows inside tile
        static const float dotColRatio[7] = {0.25f, 0.75f, 0.25f, 0.75f, 0.25f, 0.75f, 0.5f};
        static const float dotRowRatio[7] = {0.2f,  0.2f,  0.4f,  0.4f,  0.6f,  0.6f,  0.8f};

        for (std::size_t y = 0; y < _drawnTiles.size(); ++y) {
            for (std::size_t x = 0; x < _drawnTiles[y].size(); ++x) {
                const auto& rect = _drawnTiles[y][x];
                rect.Draw(_colors.getTileColor());
                rect.DrawLines(_colors.getTileBorderColor());

                if (y >= _tileResources.size() || x >= _tileResources[y].size())
                    continue;
                const auto& res    = _tileResources[y][x];
                const int   tileW  = static_cast<int>(rect.width);
                const int   tileH  = static_cast<int>(rect.height);
                const int   radius = std::max(2, std::min(tileW, tileH) / 8);
                for (int r = 0; r < 7; ++r) {
                    if (r >= static_cast<int>(res.size()) || res[r] == 0)
                        continue;
                    const int cx = static_cast<int>(rect.x) + static_cast<int>(dotColRatio[r] * tileW);
                    const int cy = static_cast<int>(rect.y) + static_cast<int>(dotRowRatio[r] * tileH);
                    DrawCircle(cx, cy, radius, resourceColors[r]);
                }
            }
        }
    }

    void Renderer::displayTiles(const Server& server) {
        updateTiles(server);
        drawTiles();
    }

    void Renderer::updateBroadcasters(std::vector<int> vec, Server& server) {
        const int                       offsetX        = 10;
        const int                       offsetY        = 110;
        const int                       availableWidth = (_window.GetWidth() / 2) - 2 * 10;
        const std::pair<size_t, size_t> mapSize        = server.world().getMapSize();
        const int                       tileSize       = std::min(availableWidth / (int)mapSize.first, (_window.GetHeight() - 120) / (int)mapSize.second);

        for (int i = 0; i < (int)vec.size(); i++) {
            Player    player  = server.players().getPlayer()[vec[i]];
            const int x       = player.getPos().first.first;
            const int y       = player.getPos().first.second;
            const int playerX = offsetX + x * tileSize + tileSize / 2;
            const int playerY = offsetY + y * tileSize + tileSize / 2;

            if (player.getIsAlive() == true)
                _broadcasters.push_back({playerX, playerY, std::chrono::high_resolution_clock::now()});
        }
    }

    void Renderer::updateIncreasers(std::vector<int> vec, Server& server) {
        const int                       offsetX        = 10;
        const int                       offsetY        = 110;
        const int                       availableWidth = (_window.GetWidth() / 2) - 2 * 10;
        const std::pair<size_t, size_t> mapSize        = server.world().getMapSize();
        const int                       tileSize       = std::min(availableWidth / (int)mapSize.first, (_window.GetHeight() - 120) / (int)mapSize.second);

        for (int i = 0; i < (int)vec.size(); i++) {
            Player    player  = server.players().getPlayer()[vec[i]];
            const int x       = player.getPos().first.first;
            const int y       = player.getPos().first.second;
            const int playerX = offsetX + x * tileSize + tileSize / 2;
            const int playerY = offsetY + y * tileSize + tileSize / 2;

            if (player.getIsAlive() == true)
                _increasers.push_back({playerX, playerY, std::chrono::high_resolution_clock::now()});
        }
    }

    void Renderer::drawBroadcastCircles() {
        const float MAX_LIFETIME_S = 5.0f;
        const float MAX_RADIUS     = 80.0f;
        auto        now            = std::chrono::high_resolution_clock::now();

        _broadcasters.erase(std::remove_if(_broadcasters.begin(), _broadcasters.end(),
                                           [&](const BroadcastCircle& c) {
                                               float elapsed = std::chrono::duration<float>(now - c.spawnTime).count();
                                               return elapsed >= MAX_LIFETIME_S;
                                           }),
                            _broadcasters.end());
        BeginBlendMode(BLEND_ALPHA);
        for (const BroadcastCircle& c : _broadcasters) {
            float         elapsed = std::chrono::duration<float>(now - c.spawnTime).count();
            float         t       = elapsed / MAX_LIFETIME_S;
            float         radius  = t * MAX_RADIUS;
            unsigned char alpha   = static_cast<unsigned char>((1.0f - t) * 255);
            Color         color   = {255, 200, 50, alpha};
            raylib::Circle().DrawBorderCircle({(float)c.centerX, (float)c.centerY}, radius - 2.0f, radius + 2.0f, 0.0f, 360.0f, 36, color);
        }
        EndBlendMode();
    }

    void Renderer::drawIncreaseCircles() {
        const float MAX_LIFETIME_S = 5.0f;
        const float MAX_RADIUS     = 80.0f;
        auto        now            = std::chrono::high_resolution_clock::now();

        _increasers.erase(std::remove_if(_increasers.begin(), _increasers.end(),
                                         [&](const BroadcastCircle& c) {
                                             float elapsed = std::chrono::duration<float>(now - c.spawnTime).count();
                                             return elapsed >= MAX_LIFETIME_S;
                                         }),
                          _increasers.end());
        BeginBlendMode(BLEND_ALPHA);
        for (const BroadcastCircle& c : _broadcasters) {
            float         elapsed = std::chrono::duration<float>(now - c.spawnTime).count();
            float         t       = elapsed / MAX_LIFETIME_S;
            float         radius  = t * MAX_RADIUS;
            unsigned char alpha   = static_cast<unsigned char>((1.0f - t) * 255);
            Color         color   = {0, 255, 0, alpha};
            raylib::Circle().DrawBorderCircle({(float)c.centerX, (float)c.centerY}, radius - 2.0f, radius + 2.0f, 0.0f, 360.0f, 36, color);
        }
        EndBlendMode();
    }

    void Renderer::displayLastLogs(const Server& server) {
        int pos = server.logs().size() - 15;
        int beg = std::max(pos, 0);
        int x   = 960;
        int y   = 400;

        _font.DrawText("LOGS :", x, y, 25, 1.0f, _colors.getTextColor());
        y += 35;
        for (; beg < server.logs().size(); beg++) {
            _font.DrawText(server.logs()[beg], x, y, 12, 1.0f, _colors.getTextColor());
            y += 22;
        }
    }

    void Renderer::displayServerElements(Server& server) {
        const int                       availableWidth = (_window.GetWidth() / 2) - 2 * 10;
        const std::pair<size_t, size_t> mapSize        = server.world().getMapSize();
        const int                       tileSize       = std::min(availableWidth / mapSize.first, (_window.GetHeight() - 120) / mapSize.second);
        displayTiles(server);
        displayPlayers(server, 10, 110, tileSize);
        _font.DrawText("ZAPFRITES", (_window.GetSize().GetX() / 2) - 3 * 50, 20, 50, 5, _colors.getTextColor());
        displayStats(server);
        displayLastLogs(server);
        displayTotalResources(server);
        displaySelectedTile(server);
        displaySelectedPlayer(server);
    }

    void Renderer::displayAll(Server& server, bool isFpsDisplayed, bool isCursorDefault) {
        _window.BeginDrawing();
        _window.ClearBackground(_colors.getBackgroundColor());
        displayServerElements(server);
        displayBasicGui(isFpsDisplayed, isCursorDefault);
        drawBroadcastCircles();
        drawIncreaseCircles();
        _window.EndDrawing();
    }

    void Renderer::close() {
        _cursor.Unload();
        _font.Unload();
        _window.Close();
    }

    raylib::Window& Renderer::getWindow() {
        return _window;
    }

    Colors& Renderer::getColors() {
        return _colors;
    }

    void Renderer::setSelectedTile(std::pair<int, int>& pair) {
        _selectedTile = pair;
    }

    void Renderer::setSelectedPlayerId(std::optional<int> i) {
        _selectedPlayerId = i;
    }

    std::vector<std::vector<raylib::Rectangle>> Renderer::getDrawnTiles() {
        return _drawnTiles;
    }
    std::optional<int> Renderer::getSelectedPlayerId() {
        return _selectedPlayerId;
    }
}
