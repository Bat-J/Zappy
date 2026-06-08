/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Worldstate
*/

#include "WorldState.hpp"

namespace zappy {
    std::pair<size_t, size_t> WorldState::getMapSize() const {
        return _mapSize;
    }
    const std::vector<Tile>& WorldState::getTiles() const {
        return _tiles;
    }
    int WorldState::getTimeReference() const {
        return _timeReference;
    }
    void WorldState::setMapSize(int x, int y) {
        _mapSize.first  = x;
        _mapSize.second = y;
    }

    void WorldState::setTimeReference(int i) {
        _timeReference = i;
    }
}