import networking
from enum import Enum


class object_type(Enum):
    player = "player"
    food = "food"
    egg = "egg"
    linemate = "linemate"
    deraumere = "deraumere"
    sibur = "sibur"
    mendiane = "mendiane"
    phiras = "phiras"
    thystame = "thystame"


def str_to_object_type(value: str) -> object_type:
    return object_type(value)


class action_type(Enum):
    forward = "Forward"
    turn_left = "Left"
    turn_right = "Right"
    look = "Look"
    inventory = "Inventory"
    broadcast = "Broadcast"
    connect_nbr = "Connect_nbr"
    fork = "Fork"
    eject = "Eject"
    take = "Take"
    set_object_down = "Set"
    start_incantation = "Incantation"


class action:
    def __init__(self, action_type: action_type, parameters: list):
        self.action_type = action_type
        self.parameters = parameters


class AI_server_interface:
    def __init__(self, host: str, port: int, team_name: str):
        self.connection = networking.AI_server_connection(host, port)
        self.team_name = team_name
        self.world_x = None
        self.world_y = None
        self.alive = None
        self.last_known_number_of_unused_slots = None
        self.last_known_inventory = None  # todo
        self.level = 1
        self.pending_response_actions = []
        self.completed_actions = []

    def connect(self) -> bool:
        """Connects to the server and performs the initial handshake."""
        if not self.connection.connect():
            return False

        # server welcome message
        if not self.connection.blocking_receive():
            return False
        while self.connection.get_message() != "WELCOME\n":
            if not self.connection.blocking_receive():
                return False

        # send team name
        self.connection.send_message(self.team_name + "\n")

        # receive slot number
        if not self.connection.blocking_receive():
            return False
        received_message = self.connection.get_message()
        if received_message is None or received_message.strip() == "ko":
            return False
        self.last_known_number_of_unused_slots = int(received_message.strip())
        if self.last_known_number_of_unused_slots < 0:
            return False  # no slots available

        # receive world dimensions
        if not self.connection.blocking_receive():
            return False
        received_message = self.connection.get_message()
        if received_message is None or received_message.strip() == "ko":
            return False
        try:
            self.world_x, self.world_y = map(int, received_message.strip().split())
        except Exception:
            return False

        self.alive = True
        return True

    def is_connected(self) -> bool:
        """Checks if the connection to the server is still alive."""
        return self.connection.is_connected()

    def is_alive(self) -> bool:
        """Checks if the Trantorian is still alive"""
        if self.alive is None:
            return False
        else:
            return self.alive

    def __parse_look_response(self, response: str):
        response = response.strip()[1:-1].strip()
        tiles = response.split(",")
        parsed_response = []
        for tile in tiles:
            tile = tile.strip()
            if tile == "":
                parsed_response.append(
                    {
                        "player": 0,
                        "food": 0,
                        "egg": 0,
                        "linemate": 0,
                        "deraumere": 0,
                        "sibur": 0,
                        "mendiane": 0,
                        "phiras": 0,
                        "thystame": 0,
                    }
                )
                continue
            objects = tile.split()
            object_counts = {}
            for obj in objects:
                if obj in object_counts:
                    object_counts[obj] += 1
                else:
                    object_counts[obj] = 1
            parsed_response.append(object_counts)
        return parsed_response

    def __parse_inventory_response(self, response: str):
        response = response.strip()[1:-1].strip()
        items = response.split(",")
        inventory = {
            "food": 0,
            "linemate": 0,
            "deraumere": 0,
            "sibur": 0,
            "mendiane": 0,
            "phiras": 0,
            "thystame": 0,
        }
        for item in items:
            item = item.strip()
            if item == "":
                continue
            try:
                obj, count = item.split()
                inventory[obj] = int(count)
            except Exception:
                continue
        return inventory

    def handle_server_messages(self):
        """Handles incoming messages from the server and updates the internal state accordingly."""
        self.connection.nonblocking_receive()
        while self.connection.is_message_available():
            message = self.connection.get_message()
            if message is None:
                continue

            # Process the message
            message = message.strip()
            if message == "dead":
                self.alive = False
                continue
            if message == "ok":
                if self.pending_response_actions:
                    completed_action = self.pending_response_actions.pop(0)
                    self.completed_actions.append(
                        {"action_type": completed_action, "success": True}
                    )
                continue
            if message == "ko":
                if self.pending_response_actions:
                    completed_action = self.pending_response_actions.pop(0)
                    self.completed_actions.append(
                        {"action_type": completed_action, "success": False}
                    )
                continue
            if message.startswith("["):
                if (
                    message.startswith("[ player")
                    or message.startswith("[ food")
                    or message.startswith("[ egg")
                    or message.startswith("[ linemate")
                    or message.startswith("[ deraumere")
                    or message.startswith("[ sibur")
                    or message.startswith("[ mendiane")
                    or message.startswith("[ phiras")
                    or message.startswith("[ thystame")
                ):
                    completed_action = self.pending_response_actions.pop(0)
                    self.completed_actions.append(
                        {
                            "action_type": completed_action,
                            "success": True,
                            "response": self.__parse_look_response(message),
                        }
                    )
                else:
                    completed_action = self.pending_response_actions.pop(0)
                    self.completed_actions.append(
                        {
                            "action_type": completed_action,
                            "success": True,
                            "response": self.__parse_inventory_response(message),
                        }
                    )
                continue
            if int(message) >= 0:
                completed_action = self.pending_response_actions.pop(0)
                self.completed_actions.append(
                    {
                        "action_type": completed_action,
                        "success": True,
                        "response": int(message),
                    }
                )
                self.last_known_number_of_unused_slots = int(message)
                continue
            if message == "Elevation underway":
                continue
            if message.startswith("Current level:"):
                self.level = int(message.split(":")[1].strip())
                self.completed_actions.append(
                    {
                        "action_type": action_type.start_incantation,
                        "success": True,
                        "response": self.level,
                    }
                )
                continue

    def send_forward_action(self):
        """Sends a forward action to the server."""
        self.connection.send_message(action_type.forward.value + "\n")
        self.pending_response_actions.append(action_type.forward)

    def send_turn_right_action(self):
        """Sends a turn right action to the server."""
        self.connection.send_message(action_type.turn_right.value + "\n")
        self.pending_response_actions.append(action_type.turn_right)

    def send_turn_left_action(self):
        """Sends a turn left action to the server."""
        self.connection.send_message(action_type.turn_left.value + "\n")
        self.pending_response_actions.append(action_type.turn_left)

    def send_look_action(self):
        """Sends a look action to the server."""
        self.connection.send_message(action_type.look.value + "\n")
        self.pending_response_actions.append(action_type.look)

    def send_inventory_action(self):
        """Sends an inventory action to the server."""
        self.connection.send_message(action_type.inventory.value + "\n")
        self.pending_response_actions.append(action_type.inventory)

    def send_broadcast_action(self, message: str):
        """Sends a broadcast action to the server with the specified message."""
        self.connection.send_message(f"{action_type.broadcast.value} {message}\n")
        self.pending_response_actions.append(action_type.broadcast)

    def send_connect_nbr_action(self):
        """Sends a connect_nbr action to the server."""
        self.connection.send_message(action_type.connect_nbr.value + "\n")
        self.pending_response_actions.append(action_type.connect_nbr)

    def send_fork_action(self):
        """Sends a fork action to the server."""
        self.connection.send_message(action_type.fork.value + "\n")
        self.pending_response_actions.append(action_type.fork)

    def send_eject_action(self):
        """Sends an eject action to the server."""
        self.connection.send_message(action_type.eject.value + "\n")
        self.pending_response_actions.append(action_type.eject)

    def send_take_action(self, object_type: object_type):
        """Sends a take action to the server for the specified object."""
        self.connection.send_message(f"{action_type.take.value} {object_type.value}\n")
        self.pending_response_actions.append(action_type.take)

    def send_set_object_down_action(self, object_type: object_type):
        """Sends a set_object_down action to the server for the specified object."""
        self.connection.send_message(
            f"{action_type.set_object_down.value} {object_type.value}\n"
        )
        self.pending_response_actions.append(action_type.set_object_down)

    def send_start_incantation_action(self):
        """Sends a start_incantation action to the server."""
        self.connection.send_message(f"{action_type.start_incantation.value}\n")
        self.pending_response_actions.append(action_type.start_incantation)
