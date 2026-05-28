/*
** EPITECH PROJECT, 2026
** Error
** File description:
** Throw exceptions
*/

#include "../include/Error.hpp"

Error::Error(std::string message) : std::runtime_error(message) {}
