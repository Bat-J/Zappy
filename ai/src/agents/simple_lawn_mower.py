import interface
from enum import Enum


class state(Enum):
    collection_looking = 1
    collection_picking = 2
    collection_moving = 3


class SimpleLawnMowerAgent:
    def __init__(self, ai_interface):
        self.ai_interface = ai_interface
        self.state = state.collection_looking

    def run(self):
        first_run = True
        waiting_for_response_message = False
        awaiting_pickup_list = []
        number_of_moves_since_last_strafe = 0
        while True:
            if not self.ai_interface.is_connected():
                print("Connection to the server lost.")
                break
            self.ai_interface.handle_server_messages()
            if not self.ai_interface.is_alive():
                print("Trantorian is no longer alive.")
                break

            if len(self.ai_interface.pending_response_actions) != 0 and not first_run:
                print("\r\033[2Kwaiting for response...", end="")
                waiting_for_response_message = True
                continue
            elif waiting_for_response_message:
                print()
                waiting_for_response_message = False

            if self.state == state.collection_looking:
                print("sending look action")
                self.ai_interface.send_look_action()
                print("switching to collection picking state")
                self.state = state.collection_picking
            elif self.state == state.collection_picking:
                if awaiting_pickup_list == []:
                    for obj, count in self.ai_interface.completed_actions[-1][
                        "response"
                    ][0].items():
                        if obj != "player" and obj != "egg" and count > 0:
                            for _ in range(count):
                                awaiting_pickup_list.append(obj)
                if awaiting_pickup_list != []:
                    print(f"sending pickup action for {awaiting_pickup_list[0]}")
                    self.ai_interface.send_take_action(
                        interface.str_to_object_type(awaiting_pickup_list[0])
                    )
                    awaiting_pickup_list.remove(awaiting_pickup_list[0])
                if awaiting_pickup_list == []:
                    print("switching to collection moving state")
                    self.state = state.collection_moving
            elif self.state == state.collection_moving:
                if number_of_moves_since_last_strafe >= min(
                    self.ai_interface.world_x, self.ai_interface.world_y
                ):
                    print("sending right action")
                    self.ai_interface.send_turn_right_action()
                    print("sending forward action")
                    self.ai_interface.send_forward_action()
                    print("sending left action")
                    self.ai_interface.send_turn_left_action()
                    number_of_moves_since_last_strafe = 0
                else:
                    print("sending forward action")
                    self.ai_interface.send_forward_action()
                    number_of_moves_since_last_strafe += 1
                print("switching to collection looking state")
                self.state = state.collection_looking

            first_run = False
