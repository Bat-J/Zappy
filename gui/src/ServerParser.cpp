/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** ServerParser
*/

#include "../include/ServerParser.hpp"

#include <sstream>
namespace zappy {
    std::vector<std::string> ServerParser::splitWords(const std::string& input) {
        std::vector<std::string> words;
        std::stringstream        stream(input);
        std::string              word;

        while (stream >> word) {
            words.push_back(word);
        }
        return words;
    }

    bool ServerParser::parsePlayerPositionLine(const std::vector<std::string>& words, Player& player) {
        if (words.size() < 5 || (words[0] != "ppo" && words[0] != "pnw")) {
            return false;
        }
        std::pair<std::pair<int, int>, Orientation> pos;
        pos.first.first  = std::atoi(words[2].c_str());
        pos.first.second = std::atoi(words[3].c_str());
        pos.second       = static_cast<Orientation>(std::atoi(words[4].c_str()));
        player.setPos(pos);
        return true;
    }
}