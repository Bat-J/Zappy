/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Player
*/

#include "Player.hpp"

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
    void Player::setLevel(u_int8_t lvl) {
        _lvl = lvl;
    }

    void Player::setFood(size_t food) {
        _totalFood = food;
    }

    void Player::setLinemate(size_t linemate) {
        _totalLinemate = linemate;
    }

    void Player::setDeraumere(size_t deraumere) {
        _totalDeraumere = deraumere;
    }

    void Player::setSibur(size_t sibur) {
        _totalSibur = sibur;
    }

    void Player::setMendiane(size_t mendiane) {
        _totalMendiane = mendiane;
    }

    void Player::setPhiras(size_t phiras) {
        _totalPhiras = phiras;
    }

    void Player::setThystame(size_t thystame) {
        _totalThystame = thystame;
    }

    void Player::setIsAlive(bool alive) {
        _isAlive = alive;
    }

    void Player::setPos(std::pair<std::pair<int, int>, Orientation> pos) {
        _pos = pos;
    }
}