/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** IObject.hpp
*/

#pragma once

#include <vector>
#include <memory>

enum EObjectType : unsigned char {
  AI,
  FOOD,
  LINEMATE,
  DERAUMERE,
  SIBUR,
  MENDIANE,
  PHIRAS,
  THYSTAME,
  EGG
};

class IObject {
public:
  virtual ~IObject() = default;

private:
  EObjectType type;
  std::vector<int> position;
};

class CTile {
public:
  CTile();
  ~CTile() = default;
  std::vector<std::unique_ptr<IObject*>> getObjectList();

private:
  std::vector<std::unique_ptr<IObject*>> objects;
};

struct STile {
    CTile tile;

    STile *up;
    STile *down;
    STile *left;
    STile *right;
};
