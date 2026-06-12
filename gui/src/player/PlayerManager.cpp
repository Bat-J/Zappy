/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** PlayerManager
*/

#include "PlayerManager.hpp"
#include "../server/ServerParser.hpp"
#include <sstream>
#include <iostream>
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
    std::vector<int> PlayerManager::takeBroadcasters() {
        return std::move(_broadcasters);
    }

    void PlayerManager::clearBroadcasters() {
        _broadcasters.clear();
    }

    void PlayerManager::addBroadcaster(int i) {
        _broadcasters.push_back(i);
    }

    std::vector<int> PlayerManager::takeIncreasers() {
        return std::move(_lvlIncreasers);
    }

    void PlayerManager::clearIncreasers() {
        _lvlIncreasers.clear();
    }

    void PlayerManager::addIncreaser(int i) {
        _lvlIncreasers.push_back(i);
    }

    void PlayerManager::checkIncrease(Socket& socket) {
        std::string fullmess;
        for (int p = 0; p < _Players.size(); p++) {
            if (_Players[p].getIsAlive() == false)
                continue;
            std::string mess = "plv #" + std::to_string(p) + "\n";
            socket.writeToSocket(mess);
            fullmess += socket.readFromSocket();
        }
        std::istringstream stream(fullmess);
        std::string        line;
        while (std::getline(stream, line)) {
            if (line.empty())
                continue;
            int n, L;
            if (std::sscanf(line.c_str(), "plv #%d %d", &n, &L) == 2) {
                if (L > _Players[n].getLevel()) {
                    addIncreaser(n);
                    _Players[n].setLevel(n);
                }
            }
        }
    }

}