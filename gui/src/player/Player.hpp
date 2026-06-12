/*
** EPITECH PROJECT, 2026
** zappy
** File description:
** Player
*/

#pragma once
#include <cstddef>
#include <sys/types.h>
#include <utility>

namespace zappy {
    enum Orientation {
        NORTH = 1,
        EAST  = 2,
        SOUTH = 3,
        WEST  = 4
    };

    class Player {
      public:
        u_int8_t                                    getLevel() const;
        size_t                                      getFood() const;
        size_t                                      getLinemate() const;
        size_t                                      getDeraumere() const;
        size_t                                      getSibur() const;
        size_t                                      getMendiane() const;
        size_t                                      getPhiras() const;
        size_t                                      getThystame() const;
        bool                                        getIsAlive() const;
        int                                         getId() const;
        std::pair<std::pair<int, int>, Orientation> getPos() const;
        void                                        setLevel(u_int8_t lvl);
        void                                        setFood(size_t food);
        void                                        setLinemate(size_t linemate);
        void                                        setDeraumere(size_t deraumere);
        void                                        setSibur(size_t sibur);
        void                                        setMendiane(size_t mendiane);
        void                                        setPhiras(size_t phiras);
        void                                        setThystame(size_t thystame);
        void                                        setIsAlive(bool alive);
        void                                        setPos(std::pair<std::pair<int, int>, Orientation> pos);

      private:
        u_int8_t                                    _lvl            = 1;
        size_t                                      _totalFood      = 0;
        size_t                                      _totalLinemate  = 0;
        size_t                                      _totalDeraumere = 0;
        size_t                                      _totalSibur     = 0;
        size_t                                      _totalMendiane  = 0;
        size_t                                      _totalPhiras    = 0;
        size_t                                      _totalThystame  = 0;
        bool                                        _isAlive        = true;
        std::pair<std::pair<int, int>, Orientation> _pos;
    };

}