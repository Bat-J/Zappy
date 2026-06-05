/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Map
*/

#pragma once

#include <vector>

namespace zappy {
    class Tile {
      public:
        std::vector<int>    Ressources;
        std::pair<int, int> Position;
    };
}