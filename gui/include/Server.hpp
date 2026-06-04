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
    void                      updateTiles();
    void                      updateRessources();
    std::pair<size_t, size_t> getMapSize() const;
    int                       getTimeReference() const;
    const std::vector<Tile>&  getTiles() const;
    size_t                    getNbPlayers() const;
    size_t                    getNbDeath() const;
    size_t                    getTotalFood() const;
    size_t                    getTotalLinemate() const;
    size_t                    getTotalDeraumere() const;
    size_t                    getTotalSibur() const;
    size_t                    getTotalMendiane() const;
    size_t                    getTotalPhiras() const;
    size_t                    getTotalThystame() const;
    void                      incrementFood(int i);
    void                      incrementLinemate(int i);
    void                      incrementDeraumere(int i);
    void                      incrementSibur(int i);
    void                      incrementMendiane(int i);
    void                      incrementPhiras(int i);
    void                      incrementThystame(int i);
    void                      resetFood(int i);
    void                      resetLinemate(int i);
    void                      resetDeraumere(int i);
    void                      resetSibur(int i);
    void                      resetMendiane(int i);
    void                      resetPhiras(int i);
    void                      resetThystame(int i);

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
