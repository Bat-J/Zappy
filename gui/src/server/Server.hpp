/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Server
*/

#pragma once

#include "../network/Socket.hpp"
#include "../player/PlayerManager.hpp"
#include "../resources/ResourceTracker.hpp"
#include "../world/WorldState.hpp"
#include <mutex>
#include <vector>

namespace zappy {
    class Server {
      public:
        Server(Socket& socket) : _socket(socket) {}
        ~Server();

        void        getInfo();
        void        handleServerUpdate();
        void        updateTiles();
        void        updateResources();
        std::mutex& getMutex() {
            return _dataMutex;
        }
        const WorldState& world() const {
            return _world;
        }
        PlayerManager& players() {
            return _players;
        }
        const PlayerManager& players() const {
            return _players;
        }
        const ResourceTracker& resources() const {
            return _resources;
        }
        const std::vector<std::string> logs() const {
            return _logs;
        }

      private:
        Socket&                  _socket;
        WorldState               _world;
        PlayerManager            _players;
        ResourceTracker          _resources;
        std::mutex               _dataMutex;
        std::vector<std::string> _logs;
    };
}
