/*
** EPITECH PROJECT, 2026
** G-NWP-400-LIL-4-1-myftp-5
** File description:
** Socket
*/

#pragma once

#include <string>
#include <cstdint>

namespace zappy {

    class Socket {
      public:
        Socket();
        ~Socket();
        void        createSocket(uint16_t port);
        void        createSocket();
        void        createSocket(Socket& socket);
        void        closeSocket();
        void        writeToSocket(const std::string& message);
        std::string readFromSocket();
        bool        acceptConnection();
        void        listenForConnections(int maxConnections);
        bool        connectTo(const uint32_t ipAddress, uint16_t port);
        std::string getLocalIP() const;
        bool        CheckIfOpen() const;
        int         getSocket() const {
            return _socketFD;
        }
        int getPort() const {
            return _port;
        }

      protected:
      private:
        int      _socketFD;
        uint16_t _port;
    };
}