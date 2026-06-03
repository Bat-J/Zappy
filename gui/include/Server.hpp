/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Server
*/

#pragma once
#include "Socket.hpp"
#include "Tile.hpp"
#include <utility>
#include <vector>
class Server {
  public:
    Server(Socket& socket) : _socket(socket) {};
    ~Server();
    void                getInfo();
    std::pair<int, int> getMapSize() const {
        return _mapSize;
    }
    int getTimeReference() const {
        return _timeReference;
    }
    const std::vector<Tile>& getTiles() const {
        return _tiles;
    }

  private:
    Socket&             _socket;
    std::pair<int, int> _mapSize;
    int                 _timeReference;
    std::vector<Tile>   _tiles;
};
