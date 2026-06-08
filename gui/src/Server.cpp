/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Server
*/

#include "../include/Server.hpp"
#include "../include/ServerParser.hpp"
#include <cstdlib>
#include <sstream>
#include <string>
#include <vector>

namespace zappy {

    Server::~Server() {}

    void Server::handleServerUpdate() {
        const std::string                     message = _socket.readFromSocket();
        std::vector<std::string>              lines;
        std::vector<std::vector<std::string>> wordsByLine;
        std::stringstream                     stream(message);
        std::string                           line;
        ServerParser                          parser;

        while (std::getline(stream, line)) {
            if (!line.empty()) {
                lines.push_back(line);

                wordsByLine.push_back(parser.splitWords(line));
            }
        }
        for (std::size_t i = 0; i < wordsByLine.size(); ++i) {
            if (wordsByLine[i].empty()) {
                continue;
            }
            if ((wordsByLine[i][0] == "ppo" || wordsByLine[i][0] == "pnw") && wordsByLine[i].size() > 4) {
                const int number  = std::atoi(wordsByLine[i][1].substr(1).c_str());
                int       oldSize = _players.getPlayer().size();
                if (static_cast<std::size_t>(number) >= _players.getPlayer().size()) {
                    _players.getPlayer().resize(static_cast<std::size_t>(number) + 1);
                    for (; oldSize < _players.getPlayer().size(); oldSize++)
                        _players.checkAlive(oldSize, _socket);
                }
                parser.parsePlayerPositionLine(wordsByLine[i], _players.getPlayer()[number]);
                _players.getPlayer()[number].setIsAlive(true);
            }
            if (wordsByLine[i][0] == "pdi") {
                const int number = std::atoi(wordsByLine[i][1].substr(1).c_str());
                _players.getPlayer()[number].setIsAlive(false);
            }
        }
        _players.getAllAlive();
        _players.updatePlayerpos(_socket);
    }

    void Server::updateTiles() {
        _socket.writeToSocket("mct\n");
        std::string              new_tiles = _socket.readFromSocket();
        std::stringstream        stream(new_tiles);
        std::string              token;
        std::vector<std::string> _tokens;

        _tokens.clear();
        std::vector<Tile> _tiles = world().getTiles();
        _tiles.clear();
        while (stream >> token) {
            _tokens.push_back(token);
        }
        for (std::size_t i = 0; i < _tokens.size(); ++i) {
            if (_tokens[i] == "bct" && i + 9 < _tokens.size()) {
                const size_t tileIndex = _tiles.size();
                _tiles.emplace_back();
                int x = std::stoi(_tokens[i + 1]);
                int y = std::stoi(_tokens[i + 2]);
                _tiles[tileIndex].setPosition(x, y);
                std::vector<int> resources;
                for (size_t resourceIndex = 3; resourceIndex <= 9; ++resourceIndex) {
                    resources.push_back(std::stoi(_tokens[i + resourceIndex]));
                }
                _tiles[tileIndex].setRessources(resources);
                i += 9;
            }
        }
    }

    void Server::updateResources() {
        return _resources.updateResources(world().getTiles());
    }

    void Server::getInfo() {
        _socket.writeToSocket("GRAPHIC\n");
        std::string              infos = _socket.readFromSocket();
        std::stringstream        stream(infos);
        std::string              token;
        std::vector<std::string> _tokens;

        _tokens.clear();
        std::vector<Tile> tiles = world().getTiles();
        tiles.clear();
        while (stream >> token) {
            _tokens.push_back(token);
        }
        for (std::size_t i = 0; i < _tokens.size(); ++i) {
            if (_tokens[i] == "msz" && i + 2 < _tokens.size()) {
                int x = std::stoi(_tokens[i + 1]);
                int y = std::stoi(_tokens[i + 2]);
                _world.setMapSize(x, y);
                i += 2;
            } else if (_tokens[i] == "sgt" && i + 1 < _tokens.size()) {
                _world.setTimeReference(std::stoi(_tokens[i + 1]));
                i += 1;
            } else if (_tokens[i] == "bct" && i + 9 < _tokens.size()) {
                const size_t tileIndex = tiles.size();
                tiles.emplace_back();
                int x = std::stoi(_tokens[i + 1]);
                int y = std::stoi(_tokens[i + 2]);
                tiles[tileIndex].setPosition(x, y);
                std::vector<int> resources;
                for (size_t resourceIndex = 3; resourceIndex <= 9; ++resourceIndex) {
                    resources.push_back(std::stoi(_tokens[i + resourceIndex]));
                }
                tiles[tileIndex].setRessources(resources);
                i += 9;
            }
        }
    }
}