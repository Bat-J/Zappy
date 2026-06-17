/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** PlayerManager
*/

#include "PlayerManager.hpp"

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

}