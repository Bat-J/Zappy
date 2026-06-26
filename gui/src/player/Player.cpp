/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Player
*/

#include "Player.hpp"
#include <cstdio>
#include <string>
#include <vector>
#include <sstream>

namespace zappy {

    u_int8_t Player::getLevel() const {
        return _lvl;
    }

    size_t Player::getFood() const {
        return _totalFood;
    }

    size_t Player::getLinemate() const {
        return _totalLinemate;
    }

    size_t Player::getDeraumere() const {
        return _totalDeraumere;
    }

    size_t Player::getSibur() const {
        return _totalSibur;
    }

    size_t Player::getMendiane() const {
        return _totalMendiane;
    }

    size_t Player::getPhiras() const {
        return _totalPhiras;
    }

    size_t Player::getThystame() const {
        return _totalThystame;
    }

    bool Player::getIsAlive() const {
        return _isAlive;
    }

    std::pair<std::pair<int, int>, Orientation> Player::getPos() const {
        return _pos;
    }

    int Player::getId() const {
        return _id;
    }
    void Player::setLevel(u_int8_t lvl) {
        _lvl = lvl;
    }

    void Player::setIsAlive(bool alive) {
        _isAlive = alive;
    }

    void Player::setPos(std::pair<std::pair<int, int>, Orientation> pos) {
        _pos = pos;
    }

    void Player::setId() {
        static int id = 0;
        _id           = id;
        id++;
    }

    std::string Player::getTeam() const {
        return _team;
    }

    void Player::setTeam(std::string& name) {
        _team = name;
    }

    void Player::updateResources(int i, Socket& socket) {
        const std::string mess = "pin #" + std::to_string(i) + "\n";
        socket.writeToSocket(mess);

        std::string resp = socket.readFromSocket();
        while (resp.empty())
            resp = socket.readFromSocket();
        std::stringstream        ss(resp);
        std::string              word;
        std::vector<std::string> tokens;
        while (ss >> word) {
            tokens.push_back(word);
        }
        if (tokens.size() >= 10 && tokens[0] == "pin") {
            _totalFood      = std::stoi(tokens[4]);
            _totalLinemate  = std::stoi(tokens[5]);
            _totalDeraumere = std::stoi(tokens[6]);
            _totalSibur     = std::stoi(tokens[7]);
            _totalMendiane  = std::stoi(tokens[8]);
            _totalPhiras    = std::stoi(tokens[9]);
            if (tokens.size() > 10) {
                _totalThystame = std::stoi(tokens[10]);
            }
        }
    }
}
