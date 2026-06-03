/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** Core
*/

#include "../include/Core.hpp"
#include <Keyboard.hpp>
#include <algorithm>
#include <raylib.h>

void Core::setupColors() {
    _ressourceColors = {RED, GREEN, BLUE, YELLOW, PURPLE, ORANGE, PINK, BROWN, GRAY, DARKGRAY, LIGHTGRAY, MAROON, LIME, SKYBLUE, VIOLET, BEIGE, DARKBROWN, GOLD};
    std::vector<raylib::Color> secondColors = {DARKGREEN, DARKBLUE, DARKPURPLE, DARKBROWN, DARKGRAY, GRAY, LIGHTGRAY, BLACK};
    _ressourceColors.insert(_ressourceColors.end(), secondColors.begin(), secondColors.end());
    _TileColor       = GRAY;
    _TileBorderColor = DARKGRAY;
}

void Core::createWindow() {
    _window.Init(1920, 1080, "Zappy GUI", 0, LOG_NONE);
    _window.SetTargetFPS(60);
    _window.SetFullscreen(_isFullscreen);
    logo   = LoadImage("assets/logo.png");
    cursor = LoadTexture("assets/cursor.png");
    _window.SetIcon(logo);
    logo.Unload();
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
    if (raylib::Keyboard::IsKeyPressed(KEY_UP)) {
        _tileColorIndex = (_tileColorIndex + 1) % _ressourceColors.size();
        _TileColor      = _ressourceColors[_tileColorIndex];
    }
    if (raylib::Keyboard::IsKeyPressed(KEY_DOWN)) {
        if (_tileColorIndex == 0)
            _tileColorIndex = _ressourceColors.size();
        _tileColorIndex = (_tileColorIndex - 1) % _ressourceColors.size();
        _TileColor      = _ressourceColors[_tileColorIndex];
    }
    if (raylib::Keyboard::IsKeyPressed(KEY_RIGHT)) {
        _tileBorderColorIndex = (_tileBorderColorIndex + 1) % _ressourceColors.size();
        _TileBorderColor      = _ressourceColors[_tileBorderColorIndex];
    }
    if (raylib::Keyboard::IsKeyPressed(KEY_LEFT)) {
        if (_tileBorderColorIndex == 0)
            _tileBorderColorIndex = _ressourceColors.size();
        _tileBorderColorIndex = (_tileBorderColorIndex - 1) % _ressourceColors.size();
        _TileBorderColor      = _ressourceColors[_tileBorderColorIndex];
    }
    if (raylib::Keyboard::IsKeyPressed(KEY_KP_ADD)) {
        _backgroundColorIndex = (_backgroundColorIndex + 1) % _ressourceColors.size();
        _BackgroundColor      = _ressourceColors[_backgroundColorIndex];
    }
    if (raylib::Keyboard::IsKeyPressed(KEY_KP_SUBTRACT)) {
        if (_backgroundColorIndex == 0)
            _backgroundColorIndex = _ressourceColors.size();
        _backgroundColorIndex = (_backgroundColorIndex - 1) % _ressourceColors.size();
        _BackgroundColor      = _ressourceColors[_backgroundColorIndex];
    }
}

void Core::displayBasicGui() {
    if (_isFpsDisplayed == true) {
        _window.DrawFPS(20, 20);
    }
    if (_isCursorDefault == false) {
        _window.HideCursor();
        raylib::Vector2 pos = raylib::Mouse::GetPosition();
        cursor.Draw(static_cast<int>(pos.x), static_cast<int>(pos.y), WHITE);
    } else {
        _window.ShowCursor();
    }
}

void Core::displayServerElements(const Server& server) {
    const auto mapSize         = server.getMapSize();
    const int  mapWidth        = std::max(1, mapSize.first);
    const int  mapHeight       = std::max(1, mapSize.second);
    const int  hudHeight       = 100;
    const int  padding         = 10;
    const int  tileSize        = std::min((_window.GetWidth() - 2 * padding) / mapWidth, (_window.GetHeight() - hudHeight - 2 * padding) / mapHeight);
    const int  tileSizeClamped = std::max(1, tileSize);
    const int  mapPixelWidth   = mapWidth * tileSizeClamped;
    const int  mapPixelHeight  = mapHeight * tileSizeClamped;
    const int  offsetX         = std::max(0, (_window.GetWidth() - mapPixelWidth) / 2);
    const int  offsetY         = hudHeight + std::max(0, (_window.GetHeight() - hudHeight - mapPixelHeight) / 2);

    for (int y = 0; y < mapHeight; ++y) {
        for (int x = 0; x < mapWidth; ++x) {
            const int         rectX = offsetX + x * tileSizeClamped;
            const int         rectY = offsetY + y * tileSizeClamped;

            raylib::Rectangle rect(rectX, rectY, tileSizeClamped - 1, tileSizeClamped - 1);
            rect.Draw(_TileColor);
            rect.DrawLines(_TileBorderColor);
        }
    }
}

int Core::Run(Socket& socket) {
    setupColors();
    createWindow();
    Server server(socket);
    server.getInfo();
    while (!_window.ShouldClose()) {
        _window.BeginDrawing();
        _window.ClearBackground(_BackgroundColor);
        displayServerElements(server);
        displayBasicGui();
        handleEvents();
        _window.EndDrawing();
    }
    cursor.Unload();
    _window.Close();
    return 0;
}