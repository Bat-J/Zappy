/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Server
*/

#include "../include/Server.hpp"
#include <sstream>
#include <string>

Server::~Server() {}

std::pair<size_t, size_t> Server::getMapSize() const {
    return _mapSize;
}

int Server::getTimeReference() const {
    return _timeReference;
}

const std::vector<Tile>& Server::getTiles() const {
    return _tiles;
}
size_t Server::getNbPlayers() const {
    return _nbPlayers;
}

size_t Server::getNbDeath() const {
    return _nbDeath;
}

size_t Server::getTotalFood() const {
    return _totalFood;
}

size_t Server::getTotalLinemate() const {
    return _totalLinemate;
}

size_t Server::getTotalDeraumere() const {
    return _totalDeraumere;
}

size_t Server::getTotalSibur() const {
    return _totalSibur;
}
size_t Server::getTotalMendiane() const {
    return _totalMendiane;
}
size_t Server::getTotalPhiras() const {
    return _totalPhiras;
}
size_t Server::getTotalThystame() const {
    return _totalThystame;
}

void Server::incrementFood(int i) {
    _totalFood += static_cast<size_t>(i);
}

void Server::incrementLinemate(int i) {
    _totalLinemate += static_cast<size_t>(i);
}

void Server::incrementDeraumere(int i) {
    _totalDeraumere += static_cast<size_t>(i);
}

void Server::incrementSibur(int i) {
    _totalSibur += static_cast<size_t>(i);
}

void Server::incrementMendiane(int i) {
    _totalMendiane += static_cast<size_t>(i);
}

void Server::incrementPhiras(int i) {
    _totalPhiras += static_cast<size_t>(i);
}

void Server::incrementThystame(int i) {
    _totalThystame += static_cast<size_t>(i);
}

void Server::resetFood(int i) {
    _totalFood = i;
}

void Server::resetLinemate(int i) {
    _totalLinemate = i;
}

void Server::resetDeraumere(int i) {
    _totalDeraumere = i;
}

void Server::resetSibur(int i) {
    _totalSibur = i;
}

void Server::resetMendiane(int i) {
    _totalMendiane = i;
}

void Server::resetPhiras(int i) {
    _totalPhiras = i;
}

void Server::resetThystame(int i) {
    _totalThystame = i;
}

void Server::updateTiles() {
    _socket.writeToSocket("mct\n");
    std::string              new_tiles = _socket.readFromSocket();
    std::stringstream        stream(new_tiles);
    std::string              token;
    std::vector<std::string> _tokens;

    _tokens.clear();
    _tiles.clear();
    while (stream >> token) {
        _tokens.push_back(token);
    }
    for (std::size_t i = 0; i < _tokens.size(); ++i) {
        if (_tokens[i] == "bct" && i + 9 < _tokens.size()) {
            const size_t tileIndex = _tiles.size();
            _tiles.emplace_back();
            _tiles[tileIndex].Position.first  = std::stoi(_tokens[i + 1]);
            _tiles[tileIndex].Position.second = std::stoi(_tokens[i + 2]);
            for (size_t resourceIndex = 3; resourceIndex <= 9; ++resourceIndex) {
                _tiles[tileIndex].Ressources.push_back(std::stoi(_tokens[i + resourceIndex]));
            }
            i += 9;
        }
    }
}

void Server::updateRessources() {
    updateTiles();
    resetFood(0);
    resetDeraumere(0);
    resetLinemate(0);
    resetMendiane(0);
    resetPhiras(0);
    resetSibur(0);
    resetThystame(0);

    for (size_t i = 0; i < getTiles().size(); ++i) {
        const auto& resources = getTiles()[i].Ressources;
        for (size_t j = 0; j < resources.size() && j < 7; ++j) {
            switch (j) {
                case 0: incrementFood(resources[j]); break;
                case 1: incrementLinemate(resources[j]); break;
                case 2: incrementDeraumere(resources[j]); break;
                case 3: incrementSibur(resources[j]); break;
                case 4: incrementMendiane(resources[j]); break;
                case 5: incrementPhiras(resources[j]); break;
                case 6: incrementThystame(resources[j]); break;
                default: break;
            }
        }
    }
}

void Server::getInfo() {
    _socket.writeToSocket("GRAPHIC\n");
    std::string              infos = _socket.readFromSocket();
    std::stringstream        stream(infos);
    std::string              token;
    std::vector<std::string> _tokens;

    _tokens.clear();
    _tiles.clear();
    while (stream >> token) {
        _tokens.push_back(token);
    }
    for (std::size_t i = 0; i < _tokens.size(); ++i) {
        if (_tokens[i] == "msz" && i + 2 < _tokens.size()) {
            _mapSize.first  = std::stoi(_tokens[i + 1]);
            _mapSize.second = std::stoi(_tokens[i + 2]);
            i += 2;
        } else if (_tokens[i] == "sgt" && i + 1 < _tokens.size()) {
            _timeReference = std::stoi(_tokens[i + 1]);
            i += 1;
        } else if (_tokens[i] == "bct" && i + 9 < _tokens.size()) {
            const size_t tileIndex = _tiles.size();
            _tiles.emplace_back();
            _tiles[tileIndex].Position.first  = std::stoi(_tokens[i + 1]);
            _tiles[tileIndex].Position.second = std::stoi(_tokens[i + 2]);
            for (size_t resourceIndex = 3; resourceIndex <= 9; ++resourceIndex) {
                _tiles[tileIndex].Ressources.push_back(std::stoi(_tokens[i + resourceIndex]));
            }
            i += 9;
        }
    }
}