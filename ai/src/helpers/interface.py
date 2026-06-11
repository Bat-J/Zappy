import helpers.networking as networking
from enum import Enum
from helpers.can_cast import can_cast
from random import randint


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
    was_ejected = "Was_ejected"
    take = "Take"
    set_object_down = "Set"
    start_incantation = "Incantation"


class cardinal_direction(Enum):
    north = 1
    east = 2
    south = 3
    west = 4

    def turn_right(self):
        return cardinal_direction((self.value % 4) + 1)

    def turn_left(self):
        return cardinal_direction((self.value - 2) % 4 + 1)


class direction(Enum):
    none = 0
    north = 1
    northwest = 2
    west = 3
    southwest = 4
    south = 5
    southeast = 6
    east = 7
    northeast = 8

    def add_cardinal_direction(self, cardinal_direction: cardinal_direction):
        return direction((self.value + cardinal_direction.value * 2 - 1) % 8 + 1)

    def sub_cardinal_direction(self, cardinal_direction: cardinal_direction):
        return direction((self.value - cardinal_direction.value * 2 - 1) % 8 + 1)


def parse_direction(direction_value: int) -> direction:
    if direction_value == 0:
        return direction.none
    return direction((direction_value - 1) % 8 + 1)


class AI_server_interface:
    def __init__(self, host: str, port: int, team_name: str):
        self.connection = networking.AI_server_connection(host, port)
        self.team_name = team_name
        self.world_x_size = None
        self.world_y_size = None
        self.alive = None
        self.last_known_number_of_unused_slots = None
        self.last_known_inventory = None  # todo
        self.level = 1
        self.pending_response_actions = []
        self.completed_actions = []
        self.message_history = []
        self.relative_direction = cardinal_direction.north
        self.relative_x_position = 0
        self.relative_y_position = 0

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
            self.world_x_size, self.world_y_size = map(
                int, received_message.strip().split()
            )
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

    def __parse_message(self, message: str):
        if message is None:
            pass
        message = message.strip()

        if message == "dead":
            self.alive = False
            pass
        elif message == "ok":
            if self.pending_response_actions:
                completed_action = self.pending_response_actions.pop(0)
                self.completed_actions.append(
                    {"action_type": completed_action, "success": True}
                )
                if completed_action == action_type.turn_right:
                    self.relative_direction = cardinal_direction(
                        (self.relative_direction.value % 4) + 1
                    )
                elif completed_action == action_type.turn_left:
                    self.relative_direction = cardinal_direction(
                        (self.relative_direction.value - 2) % 4 + 1
                    )
                elif completed_action == action_type.forward:
                    if self.relative_direction == cardinal_direction.north:
                        self.relative_x_position += 1
                    elif self.relative_direction == cardinal_direction.east:
                        self.relative_y_position += 1
                    elif self.relative_direction == cardinal_direction.south:
                        self.relative_x_position -= 1
                    elif self.relative_direction == cardinal_direction.west:
                        self.relative_y_position -= 1
            pass
        elif message == "ko":
            if self.pending_response_actions:
                completed_action = self.pending_response_actions.pop(0)
                self.completed_actions.append(
                    {"action_type": completed_action, "success": False}
                )
            pass
        elif message.startswith("["):
            if message.count(",") >= 3:
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
            pass
        elif can_cast(str, int) and int(message) >= 0:
            completed_action = self.pending_response_actions.pop(0)
            self.completed_actions.append(
                {
                    "action_type": completed_action,
                    "success": True,
                    "response": int(message),
                }
            )
            self.last_known_number_of_unused_slots = int(message)
            pass
        elif message == "Elevation underway":
            pass
        elif message.startswith("Current level:"):
            self.level = int(message.split(":")[1].strip())
            self.completed_actions.append(
                {
                    "action_type": action_type.start_incantation,
                    "success": True,
                    "response": self.level,
                }
            )
            pass
        elif message.startswith("message "):
            self.message_history.append(
                {
                    "text": message[11:],
                    "direction": parse_direction(
                        int(message[8:9])
                    ).add_cardinal_direction(self.relative_direction),
                }
            )
            pass
        elif message.startswith("eject: "):
            direction = parse_direction(int(message[7:])).add_cardinal_direction(
                self.relative_direction
            )
            if direction == direction.north:
                self.relative_x_position -= 1
            elif direction == direction.northwest:
                self.relative_x_position -= 1
                self.relative_y_position -= 1
            elif direction == direction.west:
                self.relative_y_position -= 1
            elif direction == direction.southwest:
                self.relative_x_position += 1
                self.relative_y_position -= 1
            elif direction == direction.south:
                self.relative_x_position += 1
            elif direction == direction.southeast:
                self.relative_x_position += 1
                self.relative_y_position += 1
            elif direction == direction.east:
                self.relative_y_position += 1
            elif direction == direction.northeast:
                self.relative_x_position -= 1
                self.relative_y_position += 1
            self.completed_actions.append(
                {
                    "action_type": action_type.was_ejected,
                    "direction": direction,
                }
            )
            pass

    def handle_server_messages_blocking(self):
        """Handles incoming messages from the server and updates the internal state accordingly."""
        self.connection.blocking_receive()
        while self.connection.is_message_available():
            message = self.connection.get_message()
            if message is not None:
                self.__parse_message(message)

    def handle_server_messages_nonblocking(self):
        """Handles incoming messages from the server and updates the internal state accordingly."""
        self.connection.nonblocking_receive()
        while self.connection.is_message_available():
            message = self.connection.get_message()
            if message is not None:
                self.__parse_message(message)

    def block_until_all_responses_received(self):
        """Blocks until all pending response actions have received a response from the server."""
        while (
            self.pending_response_actions
            and self.connection.is_connected()
            and self.is_alive()
        ):
            self.handle_server_messages_blocking()

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

    def send_forward_action_and_block_until_response(self):
        """Sends a forward action to the server and blocks until a response is received."""
        self.send_forward_action()
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_turn_right_action_and_block_until_response(self):
        """Sends a turn right action to the server and blocks until a response is received."""
        self.send_turn_right_action()
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_turn_left_action_and_block_until_response(self):
        """Sends a turn left action to the server and blocks until a response is received."""
        self.send_turn_left_action()
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_look_action_and_block_until_response(self):
        """Sends a look action to the server and blocks until a response is received."""
        self.send_look_action()
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_inventory_action_and_block_until_response(self):
        """Sends an inventory action to the server and blocks until a response is received."""
        self.send_inventory_action()
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_broadcast_action_and_block_until_response(self, message: str):
        """Sends a broadcast action to the server with the specified message and blocks until a response is received."""
        self.send_broadcast_action(message)
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_connect_nbr_action_and_block_until_response(self):
        """Sends a connect_nbr action to the server and blocks until a response is received."""
        self.send_connect_nbr_action()
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_fork_action_and_block_until_response(self):
        """Sends a fork action to the server and blocks until a response is received."""
        self.send_fork_action()
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_eject_action_and_block_until_response(self):
        """Sends an eject action to the server and blocks until a response is received."""
        self.send_eject_action()
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_take_action_and_block_until_response(self, object_type: object_type):
        """Sends a take action to the server for the specified object and blocks until a response is received."""
        self.send_take_action(object_type)
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_set_object_down_action_and_block_until_response(
        self, object_type: object_type
    ):
        """Sends a set_object_down action to the server for the specified object and blocks until a response is received."""
        self.send_set_object_down_action(object_type)
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def send_start_incantation_action_and_block_until_response(self):
        """Sends a start_incantation action to the server and blocks until a response is received."""
        self.send_start_incantation_action()
        self.block_until_all_responses_received()
        return self.completed_actions[-1]

    def replace_spaces_with_underscores(self, message: str) -> str:
        """Replaces spaces in the message with underscores."""
        return message.replace(" ", "_")

    def generate_fake_broadcast_message(self, message: str = ""):
        """Generates a fake broadcast message from the server with the specified message and direction. Message parameter is optional"""
        if message != "":
            random_action = randint(0, 11)
            if random_action == 0:
                # return the message as is
                return self.replace_spaces_with_underscores(message)
            elif random_action == 1 and any(char.isdigit() for char in message):
                # replace digits in the message with random digits
                for char in message:
                    if char.isdigit():
                        random_digit = randint(0, 9)
                        message = message.replace(char, str(random_digit), 1)
                return self.replace_spaces_with_underscores(message)
            elif random_action == 2:
                # censor a random word in the message
                words = message.split()
                if words:
                    random_word_index = randint(0, len(words) - 1)
                    words[random_word_index] = "******"
                    return "_".join(words)
            elif random_action == 3:
                # put the message in uppercase
                return self.replace_spaces_with_underscores(message.upper())
            elif random_action == 4:
                # return a message indicating that an incantation is underway
                return "Incantation_underway"
            elif random_action == 5:
                # return a random level message
                return "Current_level:_" + str(randint(1, 8))
            elif random_action == 6:
                # return a very long message to confuse other agents
                return self.replace_spaces_with_underscores("A chair is a type of seat, "
                "typically designed for one person and consisting of one or more legs, "
                "a flat or slightly angled seat, and a back-rest. It may be made of wood, "
                "metal, or synthetic materials, and may be padded or upholstered in various colors and fabrics. "
                "Chairs vary in design. An armchair has armrests fixed to the seat; "
                "a recliner is upholstered and features a mechanism that lowers the chair's back and raises into place a footrest; "
                "a rocking chair has legs fixed to two long curved slats; and a wheelchair has wheels fixed to an axis under the seat.")
            elif random_action == 7:
                # cesar code the message with a random shift
                shift = randint(1, 25)
                encrypted_message = ""
                for char in message:
                    if char.isalpha():
                        if char.isupper():
                            offset = 65
                        else:
                            offset = 97
                        encrypted_char = chr((ord(char) - offset + shift) % 26 + offset)
                        encrypted_message += encrypted_char
                    else:
                        encrypted_message += char
                    return self.replace_spaces_with_underscores(encrypted_message)
            elif random_action == 8:
                # reverse message
                return self.replace_spaces_with_underscores(message[::-1])
            elif random_action == 9:
                # replace vowels with consonants and vice versa
                vowels = "aeiouyAEIOUY"
                consonants = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ"
                for vowel in vowels:
                    message = message.replace(vowel, consonants[randint(0, len(consonants) - 1)])
                return self.replace_spaces_with_underscores(message)
            elif random_action == 10:
                # replace consonants with vowels and vice versa
                vowels = "aeiouyAEIOUY"
                consonants = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ"
                for consonant in consonants:
                    message = message.replace(consonant, vowels[randint(0, len(vowels) - 1)])
                return self.replace_spaces_with_underscores(message)
            elif random_action == 11:
                # return a message with all the keywords of the project
                return self.replace_spaces_with_underscores("zappy zapfrites server gui ai network team tile map resource "
                "winner mover simple lawn mower winning player maximum elevation 0 1 2 3 4 5 6 7 8 9 environment geography "
                "game world inhabitant trantor board up down left right planet food stone linemate deraumere sibur mendiane "
                "phiras thystame spawn time limit width height density feed trantorian collecting objective activity individual "
                "eat creature see survive unit live ritual goal everyone level incantation generation action vision look sound "
                "transmission broadcasting broadcast message direction event egg north east west south inventory fork eject position direction")
        else:
            # return a random message from the server
            random_messages = [
                "Incantation underway",
                "Current level: 2",
                "Current level: 3",
                "Current level: 4",
                "Current level: 5",
                "Current level: 6",
                "Current level: 7",
                "Current_level: 8",
                "Hello! 👋😃🚀",
            ]
            return self.replace_spaces_with_underscores(random_messages[randint(0, len(random_messages) - 1)])
