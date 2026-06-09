import helpers.interface as interface
from enum import Enum


class WinnerAgent:
    class State(Enum):
        none = 0
        collecting_ressources = 1
        egg_laying = 2
        elevating = 3

    class Collecting_State(Enum):
        moving = 1
        looking = 2
        picking = 3

    def __init__(self, ai_interface: interface.AI_server_interface):
        self.ai_interface = ai_interface
        self.state = WinnerAgent.State.none
        self.awaiting_pickup_list = []
        self.collecting_state = WinnerAgent.Collecting_State.moving

    def run(self):
        while True:
            if not self.ai_interface.is_connected():
                print("Connection to the server lost.")
                break
            self.ai_interface.handle_server_messages_nonblocking()
            if not self.ai_interface.is_alive():
                print("Trantorian is no longer alive.")
                break

            if self.state == WinnerAgent.State.none:
                self.ai_interface.send_inventory_action_and_block_until_response()
                self.ai_interface.send_look_action_and_block_until_response()
                if (self.ai_interface.last_known_inventory and self.ai_interface.last_known_inventory["food"] < 10):  # or not enough ressources for the next level
                    self.state = WinnerAgent.State.collecting_ressources
                    self.collecting_state = WinnerAgent.Collecting_State.moving
                elif (self.ai_interface.last_known_number_of_unused_slots and self.ai_interface.last_known_number_of_unused_slots > 0):
                    self.state = WinnerAgent.State.egg_laying
                else:
                    self.state = WinnerAgent.State.elevating

            elif self.state == WinnerAgent.State.collecting_ressources:
                if self.collecting_state == WinnerAgent.Collecting_State.moving:
                    self.ai_interface.goto(
                        self.ai_interface.relative_x_position + 1,
                        self.ai_interface.relative_y_position + 0,
                    )
                    self.collecting_state = WinnerAgent.Collecting_State.looking
                elif self.collecting_state == WinnerAgent.Collecting_State.looking:
                    self.awaiting_pickup_list = []
                    self.ai_interface.send_look_action_and_block_until_response()
                    for obj, count in self.ai_interface.completed_actions[-1]["response"][0].items():
                        if obj != "player" and obj != "egg" and count > 0:
                            for _ in range(count):
                                self.awaiting_pickup_list.append(obj)
                    if self.awaiting_pickup_list != []:
                        self.collecting_state = WinnerAgent.Collecting_State.picking
                    else:
                        self.collecting_state = WinnerAgent.Collecting_State.moving
                elif self.collecting_state == WinnerAgent.Collecting_State.picking:
                    self.ai_interface.send_take_action_and_block_until_response(
                        interface.str_to_object_type(self.awaiting_pickup_list[0])
                    )
                    self.awaiting_pickup_list.remove(self.awaiting_pickup_list[0])
                    if self.awaiting_pickup_list == []:
                        self.state = WinnerAgent.State.none

            elif self.state == WinnerAgent.State.egg_laying:
                self.ai_interface.send_fork_action_and_block_until_response()
                self.state = WinnerAgent.State.none

            elif self.state == WinnerAgent.State.elevating:
                self.state = WinnerAgent.State.none
                pass  # todo elevate
                
