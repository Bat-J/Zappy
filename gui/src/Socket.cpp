/*
** EPITECH PROJECT, 2026
** G-YEP-400-LIL-4-1-zappy-2
** File description:
** Socket
*/

#include "../include/Socket.hpp"
#include "../include/Error.hpp"

#include <sys/socket.h>
#include <netinet/in.h>
#include <sys/poll.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <signal.h>
#include <cstring>
#include <cerrno>

Socket::Socket() : _socketFD(-1) {}

Socket::~Socket() {
    closeSocket();
}

void Socket::createSocket(uint16_t port) {
    if (port == 0) {
        createSocket();
        return;
    }
    closeSocket();
    _port     = port;
    _socketFD = socket(AF_INET, SOCK_STREAM, 0);
    if (_socketFD == -1) {
        throw Error("Failed to create socket");
    }
    struct sockaddr_in serverAddress;
    serverAddress.sin_family      = AF_INET;
    serverAddress.sin_addr.s_addr = INADDR_ANY;
    serverAddress.sin_port        = htons(port);
    if (bind(_socketFD, reinterpret_cast<sockaddr*>(&serverAddress), sizeof(serverAddress)) == -1) {
        throw Error("Failed to bind socket");
    }
}

void Socket::createSocket() {
    closeSocket();
    _socketFD = socket(AF_INET, SOCK_STREAM, 0);
    if (_socketFD == -1) {
        throw Error("Failed to create socket");
    }
    struct sockaddr_in serverAddress;
    serverAddress.sin_family      = AF_INET;
    serverAddress.sin_addr.s_addr = INADDR_ANY;
    serverAddress.sin_port        = htons(0);
    if (bind(_socketFD, reinterpret_cast<sockaddr*>(&serverAddress), sizeof(serverAddress)) == -1) {
        throw Error("Failed to bind socket");
    }
    socklen_t serverAddressSize = sizeof(serverAddress);
    if (getsockname(_socketFD, reinterpret_cast<sockaddr*>(&serverAddress), &serverAddressSize) == -1) {
        throw Error("Failed to get socket name");
    }
    _port = ntohs(serverAddress.sin_port);
}

void Socket::createSocket(Socket& socket) {
    closeSocket();
    struct sockaddr_in serverAddress;
    serverAddress.sin_family      = AF_INET;
    serverAddress.sin_addr.s_addr = INADDR_ANY;
    serverAddress.sin_port        = htons(socket._port);
    socklen_t serverAddressSize   = sizeof(struct sockaddr_in);
    _socketFD                     = accept(socket._socketFD, reinterpret_cast<sockaddr*>(&serverAddress), &serverAddressSize);
    if (_socketFD == -1) {
        throw Error("Failed to accept client connection");
    }
}

void Socket::closeSocket() {
    if (_socketFD != -1) {
        shutdown(_socketFD, SHUT_RDWR);
        close(_socketFD);
        _socketFD = -1;
    }
}

void Socket::writeToSocket(const std::string& message) {
    write(_socketFD, message.c_str(), message.size());
}

std::string Socket::readFromSocket() {
    std::string message;
    char        chunk[1024];
    pollfd      pollFD{_socketFD, POLLIN, 0};

    while (true) {
        if (poll(&pollFD, 1, 1000) <= 0)
            break;
        const ssize_t bytesRead = recv(_socketFD, chunk, sizeof(chunk) - 1, MSG_DONTWAIT);
        if (bytesRead < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK || errno == EINTR)
                continue;
            throw Error("Failed to read from socket");
        }
        if (bytesRead == 0)
            break;
        chunk[bytesRead] = '\0';
        message.append(chunk, static_cast<std::size_t>(bytesRead));
    }
    return message;
}

bool Socket::acceptConnection() {
    struct sockaddr_in serverAddress;
    serverAddress.sin_family      = AF_INET;
    serverAddress.sin_addr.s_addr = INADDR_ANY;
    serverAddress.sin_port        = htons(_port);
    socklen_t serverAddressSize   = sizeof(struct sockaddr_in);
    pollfd    pollFD;
    pollFD.fd      = _socketFD;
    pollFD.events  = POLLIN;
    pollFD.revents = 0;
    if (poll(&pollFD, 1, 0) == -1) {
        throw Error("Failed to poll socket");
    }
    if (pollFD.revents & POLLIN) {
        int newSocketFD = accept(_socketFD, reinterpret_cast<sockaddr*>(&serverAddress), &serverAddressSize);
        if (newSocketFD == -1) {
            return (false);
        } else {
            close(_socketFD);
            _socketFD = newSocketFD;
            return (true);
        }
    } else {
        return (false);
    }
}

void Socket::listenForConnections(int maxConnections) {
    if (listen(_socketFD, maxConnections) == -1) {
        throw Error("Failed to listen on socket");
    }
}

bool Socket::connectTo(const uint32_t ipAddress, uint16_t port) {
    closeSocket();
    _socketFD = socket(AF_INET, SOCK_STREAM, 0);
    if (_socketFD == -1) {
        throw Error("Failed to create socket");
    }
    struct sockaddr_in serverAddress;
    serverAddress.sin_family      = AF_INET;
    serverAddress.sin_addr.s_addr = ipAddress;
    serverAddress.sin_port        = htons(port);
    if (connect(_socketFD, reinterpret_cast<sockaddr*>(&serverAddress), sizeof(serverAddress)) == -1) {
        return false;
    }
    return true;
}

std::string Socket::getLocalIP() const {
    struct sockaddr_in localAddress;
    socklen_t          localAddressSize = sizeof(localAddress);
    if (getsockname(_socketFD, reinterpret_cast<sockaddr*>(&localAddress), &localAddressSize) == -1) {
        throw Error("Failed to get socket name");
    }
    std::string addressString(INET_ADDRSTRLEN, '\0');
    inet_ntop(AF_INET, &localAddress.sin_addr, addressString.data(), addressString.size());
    for (char& c : addressString) {
        if (c == '.') {
            c = ',';
        }
    }
    return addressString;
}

bool Socket::CheckIfOpen() const {
    if (_socketFD == -1) {
        return false;
    }
    struct sigaction emptySignalHandler;
    struct sigaction oldSignalHandler;
    emptySignalHandler.sa_handler = SIG_IGN;
    sigemptyset(&emptySignalHandler.sa_mask);
    emptySignalHandler.sa_flags = 0;
    sigaction(SIGPIPE, &emptySignalHandler, &oldSignalHandler);
    if (write(_socketFD, "", 0) == -1 && errno != EBADF) {
        sigaction(SIGPIPE, &oldSignalHandler, NULL);
        return false;
    } else {
        sigaction(SIGPIPE, &oldSignalHandler, NULL);
        return true;
    }
}