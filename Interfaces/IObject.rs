/*
** EPITECH PROJECT, 2026
** Zapfrites
** File description:
** IObject.rs
*/

use std::rc::{Rc, Weak};
use std::vec::Vec;

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
  pub position : Vec<int>,
};

pub struct CTile {
  pub objects : Vec<Rc<Object>>,
};

pub struct STile {
    pub tile : CTile;

    pub up : Rc<STile>;
    pub down : Rc<STile>;
    pub left : Rc<STile>;
    pub right : Rc<STile>;
};
