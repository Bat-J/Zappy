/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** ServerParser
*/

#pragma once

#include <string>
#include <vector>
#include "Player.hpp"

namespace zappy {
    class ServerParser {
      public:
        static std::vector<std::string> splitWords(const std::string& input);
        static bool                     parsePlayerPositionLine(const std::vector<std::string>& words, Player& player);
    };
}