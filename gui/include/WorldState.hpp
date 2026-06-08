/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** WorldState
*/

#pragma once

#include <sys/types.h>
#include <utility>
#include <vector>
#include "Tile.hpp"

namespace zappy {
    class WorldState {
      public:
        std::pair<size_t, size_t> getMapSize() const;
        const std::vector<Tile>&  getTiles() const;
        int                       getTimeReference() const;

        void                      setMapSize(int x, int y);
        void                      setTimeReference(int i);

      private:
        std::pair<size_t, size_t> _mapSize;
        std::vector<Tile>         _tiles;
        int                       _timeReference = 0;
    };
}