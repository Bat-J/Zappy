/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Server
*/

#include "Server.hpp"
#include "ServerParser.hpp"
#include <cstdlib>
#include <mutex>
#include <sstream>
#include <string>
#include <vector>

namespace zappy {

    Server::~Server() {}

    void Server::handleServerUpdate() {
            const std::string                 message = _socket.readFromSocket();
            std::vector<std::vector<std::string>> wordsByLine;
            std::stringstream                     stream(message);
            std::string                           line;
            ServerParser                          parser;
            std::string                           dir    = "unk";

            while (std::getline(stream, line)) {
                if (!line.empty())
                    wordsByLine.push_back(parser.splitWords(line));
            }
            std::vector<Player>& players = _players.getPlayer();
            for (std::size_t i = 0; i < wordsByLine.size(); ++i) {
                if (wordsByLine[i].empty())
                    continue;
                if ((wordsByLine[i][0] == "ppo" && wordsByLine[i].size() == 5) || (wordsByLine[i][0] == "pnw" && wordsByLine[i].size() == 7)) {
                    const int number  = std::atoi(wordsByLine[i][1].substr(1).c_str());
                    int       oldSize = static_cast<int>(players.size());
                    if (static_cast<std::size_t>(number) >= players.size()) {
                        players.resize(static_cast<std::size_t>(number) + 1);
                        for (; oldSize < static_cast<int>(players.size()); oldSize++)
                            _players.checkAlive(oldSize, _socket, _logs);
                        if (wordsByLine[i][0] == "pnw")
                            _logs.push_back("New player " + std::to_string(number + 1) + " connected");
                    }
                    parser.parsePlayerPositionLine(wordsByLine[i], players[number]);
                    players[number].setIsAlive(true);
                    if (wordsByLine[i][0] == "ppo") {
                        int orientCode = std::atoi(wordsByLine[i][4].c_str());

                        if (orientCode == 1) dir = "North";
                        else if (orientCode == 2) dir = "East";
                        else if (orientCode == 3) dir = "South";
                        else if (orientCode == 4) dir = "West";
                        _logs.push_back("Player " + std::to_string(number + 1) +
                                        " moved to position (" + wordsByLine[i][2] + ", " + wordsByLine[i][3] +
                                        ") orientation " + dir);
                    }
                }
                if (wordsByLine[i][0] == "pdi") {
                    const int number = std::atoi(wordsByLine[i][1].substr(1).c_str());
                    if (static_cast<std::size_t>(number) < players.size())
                        players[number].setIsAlive(false);
                    _logs.push_back("Player " + std::to_string(number + 1) + " died");
                }
                if (wordsByLine[i][0] == "pbc") {
                    const int number  = std::atoi(wordsByLine[i][1].substr(1).c_str());
                    int       oldSize = static_cast<int>(players.size());
                    if (static_cast<std::size_t>(number) >= players.size()) {
                        players.resize(static_cast<std::size_t>(number) + 1);
                        for (; oldSize < static_cast<int>(players.size()); oldSize++)
                            _players.checkAlive(oldSize, _socket, _logs);
                    }
                    _logs.push_back("Player " + std::to_string(number + 1) + " sent the message " + wordsByLine[i][2]);
                    _players.addBroadcaster(number);
                }
            }
            for (int i = 0; i < players.size(); i++)
                _players.checkAlive(i, _socket, _logs);
            _players.getAllAlive();
            _players.updatePlayerpos(_socket, _logs);
        }

    void Server::updateTiles() {
        std::string mess = "mct\n";
        _socket.writeToSocket(mess);
        std::string              new_tiles = _socket.readFromSocket();
        std::stringstream        stream(new_tiles);
        std::string              token;
        std::vector<std::string> tokens;
        std::vector<Tile>        tiles;
        while (stream >> token)
            tokens.push_back(token);
        for (std::size_t i = 0; i < tokens.size(); ++i) {
            if (tokens[i] == "bct" && i + 9 < tokens.size()) {
                Tile tile;
                int  x = std::stoi(tokens[i + 1]);
                int  y = std::stoi(tokens[i + 2]);
                tile.setPosition(x, y);

                std::vector<int> resources;
                for (size_t r = 3; r <= 9; ++r)
                    resources.push_back(std::stoi(tokens[i + r]));
                tile.setRessources(resources);

                tiles.push_back(tile);
                i += 9;
            }
        }
        std::lock_guard<std::mutex> lock(_dataMutex);
        _world.setTiles(tiles);
    }

    void Server::updateResources() {
        _resources.updateResources(world().getTiles());
        for (int i = 0; i < _players.getPlayer().size(); i++) {
            if (_players.getPlayer()[i].getIsAlive() == false)
                continue;
            else {
                _players.getPlayer()[i].updateResources(i, _socket);
            }
        }
    }

    void Server::getInfo() {
        std::string mess = "GRAPHIC\n";
        _socket.writeToSocket(mess);
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
