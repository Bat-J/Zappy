/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** PlayerManager
*/

#pragma once

#include <string>
#include <vector>
#include "Player.hpp"
#include "Socket.hpp"

namespace zappy {
    class PlayerManager {
      public:
        std::vector<Player> getPlayer() const;
        size_t              getNbPlayers() const;
        size_t              getNbDeath() const;
        void                getAllAlive();
        void                checkAlive(int pNumber, Socket& socket);
        void                updatePlayerpos(Socket& socket);

      private:
        std::vector<Player> _Players;
        size_t              _nbPlayers = 0;
        size_t              _nbDeath   = 0;
    };
}