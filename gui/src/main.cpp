/*
** EPITECH PROJECT, 2026
** gui main
** File description:
** GUI
*/

#include <iostream>
#include <string>
#include <arpa/inet.h>
#include "network/Socket.hpp"
#include "core/Core.hpp"

int main(int argc, char** argv) {
    if (argc == 2 && std::string(argv[1]) == "--help") {
        std::cout << "Usage: ./zappy_gui -p port -h machine\n";
        return 0;
    } else if (argc != 5 || std::string(argv[1]) != "-p" || std::string(argv[3]) != "-h") {
        std::cerr << "Usage: ./zappy_gui -p port -h machine\n";
        return 84;
    }
    zappy::Socket socket;
    if (!socket.connectTo(inet_addr(argv[4]), std::stoi(argv[2]))) {
        std::cerr << "Failed to connect to server\n";
        return 84;
    }
    zappy::Core core;
    return core.Run(socket);
}
