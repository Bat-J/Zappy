/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** PlayerManager
*/

#include "PlayerManager.hpp"
#include "../server/ServerParser.hpp"
#include <sstream>
#include <string>
#include <vector>

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

    void PlayerManager::checkAlive(int pNumber, Socket& socket, std::vector<std::string>& logs) {
        std::string comm = "plv #" + std::to_string(pNumber) + "\n";
        socket.writeToSocket(comm);
        std::string                    resp = socket.readFromSocket();
        ServerParser                   parser;
        const std::vector<std::string> words = parser.splitWords(resp);

        _Players[pNumber].setIsAlive(false);
        for (std::size_t i = 0; i + 2 < words.size(); ++i) {
            if (words[i] == "plv" && words[i + 1] == "#" + std::to_string(pNumber)) {
                _Players[pNumber].setIsAlive(true);
                if (std ::stoi(words[i + 2]) > _Players[pNumber].getLevel()) {
                    _Players[pNumber].setLevel(std::stoi(words[i + 2]));
                    addIncreaser(pNumber);
                    logs.push_back("Player " + std::to_string(pNumber + 1) + " increased Lvl to " + words[i + 2]);
                }
                break;
            }
        }
        getAllAlive();
    }

    void PlayerManager::updatePlayerpos(Socket& socket, std::vector<std::string>& logs) {
        std::string dir = "unk";
        for (std::size_t i = 0; i < _Players.size(); ++i) {
            if (!_Players[i].getIsAlive()) {
                continue;
            }
            std::string comm = "ppo #" + std::to_string(i) + "\n";
            socket.writeToSocket(comm);
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
                    if (words[3].compare("1")) {
                        dir = "North";
                    }
                    if (words[3].compare("2")) {
                        dir = "East";
                    }
                    if (words[3].compare("3")) {
                        dir = "South";
                    }
                    if (words[3].compare("4")) {
                        dir = "West";
                    }
                    logs.push_back("Player " + std::to_string(i + 1) + " moved to X: " + words[2] + " Y: " + words[3] + " facing " + dir);
                }
            }
        }
    }
}
