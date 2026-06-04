/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Server
*/

#pragma once
#include "Socket.hpp"
#include "Tile.hpp"
#include <cstddef>
#include <utility>
#include <vector>
class Server {
  public:
    Server(Socket& socket) : _socket(socket) {};
    ~Server();
    void                      getInfo();
    std::pair<size_t, size_t> getMapSize() const {
        return _mapSize;
    }
    int getTimeReference() const {
        return _timeReference;
    }
    const std::vector<Tile>& getTiles() const {
        return _tiles;
    }
    size_t getNbPlayers() const {
        return _nbPlayers;
    }
    size_t getNbDeath() const {
        return _nbDeath;
    }
    size_t getTotalFood() const {
        return _totalFood;
    }
    size_t getTotalLinemate() const {
        return _totalLinemate;
    }
    size_t getTotalDeraumere() const {
        return _totalDeraumere;
    }
    size_t getTotalSibur() const {
        return _totalSibur;
    }
    size_t getTotalMendiane() const {
        return _totalMendiane;
    }
    size_t getTotalPhiras() const {
        return _totalPhiras;
    }
    size_t getTotalThystame() const {
        return _totalThystame;
    }

  private:
    Socket&                   _socket;
    std::pair<size_t, size_t> _mapSize;
    int                       _timeReference;
    std::vector<Tile>         _tiles;
    size_t                    _nbPlayers      = 0;
    size_t                    _nbDeath        = 0;
    size_t                    _totalFood      = 0;
    size_t                    _totalLinemate  = 0;
    size_t                    _totalDeraumere = 0;
    size_t                    _totalSibur     = 0;
    size_t                    _totalMendiane  = 0;
    size_t                    _totalPhiras    = 0;
    size_t                    _totalThystame  = 0;
};
