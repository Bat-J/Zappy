/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** PlayerManager
*/

#include "PlayerManager.hpp"
#include "../server/ServerParser.hpp"
#include <sstream>

namespace zappy {

    std::vector<Player>& PlayerManager::getPlayer() {
        return _Players;
    }

    const std::vector<Player>& PlayerManager::getPlayer() const {
        return _Players;
    }

    size_t PlayerManager::getNbPlayers() const {
        return _nbPlayers;
    }

    size_t PlayerManager::getNbDeath() const {
        return _nbDeath;
    }

    void PlayerManager::getAllAlive() {
        int nbAlive  = 0;
        int nbDeaths = 0;
        for (int i = 0; i < _Players.size(); i++) {
            if (_Players[i].getIsAlive())
                nbAlive++;
            else {
                nbDeaths++;
            }
        }
        _nbPlayers = nbAlive;
        _nbDeath   = nbDeaths;
    }

    void PlayerManager::checkAlive(int pNumber, Socket& socket) {
        socket.writeToSocket("plv #" + std::to_string(pNumber) + "\n");
        std::string                    resp = socket.readFromSocket();
        ServerParser                   parser;
        const std::vector<std::string> words = parser.splitWords(resp);

        _Players[pNumber].setIsAlive(false);
        for (std::size_t i = 0; i + 2 < words.size(); ++i) {
            if (words[i] == "plv" && words[i + 1] == "#" + std::to_string(pNumber)) {
                _Players[pNumber].setIsAlive(true);
                _Players[pNumber].setLevel(std::stoi(words[i + 2]));
                break;
            }
        }
        getAllAlive();
    }

    void PlayerManager::updatePlayerpos(Socket& socket) {
        for (std::size_t i = 0; i < _Players.size(); ++i) {
            if (!_Players[i].getIsAlive()) {
                continue;
            }

            socket.writeToSocket("ppo #" + std::to_string(i) + "\n");
            const std::string resp = socket.readFromSocket();
            std::stringstream stream(resp);
            std::string       token;
            ServerParser      parser;

            while (std::getline(stream, token)) {
                if (token.empty()) {
                    continue;
                }
                const auto words = parser.splitWords(token);
                if (words.size() >= 5 && words[1] == "#" + std::to_string(i) && parser.parsePlayerPositionLine(words, _Players[i])) {
                    _Players[i].setIsAlive(true);
                }
            }
        }
    }
}