/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** IObject.rs
*/

pub enum EObjectType : unsigned char {
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

pub struct Object {
  pub Type : EObjectType,
  pub position : [2]int,
};

pub struct CTile {
  pub objects : Object**,
};

pub struct STile {
    pub tile : CTile;

    pub up : STile *;
    pub down : STile *;
    pub left : STile *;
    pub right : STile *;
};
