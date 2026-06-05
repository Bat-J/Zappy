/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Player
*/

#pragma once
#include <cstddef>
#include <sys/types.h>
#include <utility>

namespace zappy {
    enum Orientation {
        NORTH = 1,
        EAST  = 2,
        SOUTH = 3,
        WEST  = 4
    };

    class Player {
      public:
        u_int8_t                                    _lvl            = 1;
        size_t                                      _totalFood      = 0;
        size_t                                      _totalLinemate  = 0;
        size_t                                      _totalDeraumere = 0;
        size_t                                      _totalSibur     = 0;
        size_t                                      _totalMendiane  = 0;
        size_t                                      _totalPhiras    = 0;
        size_t                                      _totalThystame  = 0;
        bool                                        isAlive         = true;
        std::pair<std::pair<int, int>, Orientation> _pos;
    };

}