/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Server
*/

#include "../include/Server.hpp"
#include <iostream>
#include <sstream>

Server::~Server() {}

void Server::getInfo() {
    _socket.writeToSocket("GRAPHIC\n");
    std::string              i     = _socket.readFromSocket();
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
        } else if (_tokens[i] == "sgt" && i + 1 < _tokens.size()) {
            _timeReference = std::stoi(_tokens[i + 1]);
        } else if (_tokens[i] == "bct") {
            Tile temp;
            temp.Position.first  = std::stoi(_tokens[i + 1]);
            temp.Position.second = std::stoi(_tokens[i + 2]);
            for (int j = 3; j <= 6 && i + j < _tokens.size(); ++j) {
                temp.Ressources.push_back(std::stoi(_tokens[i + j]));
            }
            _tiles.push_back(temp);
        }
    }
}