import socket


class Connection:
    def __init__(self, host: str, port: int):
        self.host = host
        self.port = port
        self.socket = None
        self.buffer = ""

    def connect(self) -> bool:
        try:
            self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.socket.connect((self.host, self.port))
            return True
        except Exception as e:
            return False

    def send_message(self, message: str) -> bool:
        if self.socket is None:
            return False
        try:
            self.socket.sendall(message.encode())
            return True
        except Exception as e:
            return False

    def receive_message(self) -> str | None:
        if self.socket is None:
            return None
        try:
            while "\n" not in self.buffer:
                data = self.socket.recv(1024)
                if not data:
                    return None
                self.buffer += data.decode()
            line, self.buffer = self.buffer.split("\n", 1)
            return line + "\n"
        except Exception as e:
            return None
