import helpers.interface as interface
from enum import Enum
import random
from helpers.can_cast import can_cast


class WinnerAgent:
    class State(Enum):
        none = 0
        collecting_ressources = 1
        egg_laying = 2
        elevating = 3

    def __init__(self, ai_interface: interface.AI_server_interface):
        self.ai_interface = ai_interface
        self.state = WinnerAgent.State.none
        self.elevation_broadcast_direction = interface.direction.southeast

    def run(self):
        while True:
            if not self.ai_interface.is_connected():
                print("Connection to the server lost.")
                break
            self.ai_interface.handle_server_messages_nonblocking()
            if not self.ai_interface.is_alive():
                print("Trantorian is no longer alive.")
                break

            for message in self.ai_interface.message_history:
                if message["text"].startswith("wawa_") and can_cast(str, int, message["text"][5:]) and int(message["text"][5:]) == self.ai_interface.level:
                    self.elevation_broadcast_direction = message["direction"].sub_cardinal_direction(self.ai_interface.relative_direction) #TODO fix direction calc
                    print(f"Received elevation broadcast from direction {message['direction']}, moving towards {self.elevation_broadcast_direction}")
                self.ai_interface.message_history.remove(message)

            if self.state == WinnerAgent.State.none:
                self.ai_interface.send_inventory_action()
                self.ai_interface.send_connect_nbr_action_and_block_until_response()
                if (self.ai_interface.last_known_inventory
                    and self.ai_interface.last_known_inventory[interface.object_type.food] < 15) or not self.ai_interface.has_enough_ressources_for_next_level():
                    self.state = WinnerAgent.State.collecting_ressources
                elif (self.ai_interface.last_known_number_of_unused_slots
                      and self.ai_interface.last_known_number_of_unused_slots == 0):
                    self.state = WinnerAgent.State.egg_laying
                else:
                    self.state = WinnerAgent.State.elevating

            if self.state == WinnerAgent.State.collecting_ressources:
                if random.random() < 1 / min(self.ai_interface.world_x_size, self.ai_interface.world_y_size) * 2: # pyright: ignore[reportArgumentType]
                    if random.random() < 0.5:
                        self.ai_interface.send_turn_left_action()
                    else:
                        self.ai_interface.send_turn_right_action()
                self.ai_interface.send_forward_action_and_block_until_response()
                self.ai_interface.send_inventory_action_and_block_until_response()
                # ressources needed for the next two incantations
                needed_ressources = self.ai_interface.subtract_objects_dicts(self.ai_interface.add_objects_dicts(self.ai_interface.get_ressources_needed_for_level(self.ai_interface.level + 1), self.ai_interface.get_ressources_needed_for_level(self.ai_interface.level)), self.ai_interface.last_known_inventory) # pyright: ignore[reportArgumentType]
                needed_ressources[interface.object_type.food] = 100 # collect all
                self.ai_interface.collect_matching_tile_ressources(needed_ressources)
                self.state = WinnerAgent.State.none

            elif self.state == WinnerAgent.State.egg_laying:
                self.ai_interface.send_fork_action_and_block_until_response()
                self.state = WinnerAgent.State.none

            elif self.state == WinnerAgent.State.elevating:
                self.ai_interface.send_inventory_action_and_block_until_response()
                if self.ai_interface.last_known_inventory and self.ai_interface.last_known_inventory[interface.object_type.food] < 5: # critically low on food abort
                    self.state = WinnerAgent.State.none
                elif not self.ai_interface.has_enough_ressources_for_next_level():
                     self.state = WinnerAgent.State.none
                else:
                    if self.ai_interface.level == 1:
                        for type, count in interface.RESSOURSE_REQUIREMENTS_FOR_LEVEL_UP[1].items():
                            for _ in range(count):
                                self.ai_interface.send_set_object_down_action_and_block_until_response(type)
                        self.ai_interface.send_start_incantation_action_and_block_until_response()
                    else:
                        self.ai_interface.send_look_action_and_block_until_response()
                        if (self.ai_interface.completed_actions
                            and self.ai_interface.completed_actions[-1]["action_type"] == interface.action_type.look 
                            and self.ai_interface.completed_actions[-1]["response"] 
                            and self.ai_interface.completed_actions[-1]["response"][0] 
                            and  self.ai_interface.completed_actions[-1]["response"][0][interface.object_type.player] >= interface.PLAYER_COUNT_REQUIREMENTS_FOR_LEVEL_UP.get(self.ai_interface.level, 0)):
                            if (self.ai_interface.completed_actions 
                                and self.ai_interface.completed_actions[-1]["action_type"] == interface.action_type.look 
                                and self.ai_interface.completed_actions[-1]["response"] 
                                and self.ai_interface.completed_actions[-1]["response"][0] 
                                and self.ai_interface.are_ressources_sufficient_for_level_up(self.ai_interface.completed_actions[-1]["response"][0], interface.RESSOURSE_REQUIREMENTS_FOR_LEVEL_UP.get(self.ai_interface.level, {}))):
                                self.ai_interface.send_start_incantation_action_and_block_until_response()
                            else:
                                for resource, required_amount in interface.RESSOURSE_REQUIREMENTS_FOR_LEVEL_UP.get(self.ai_interface.level, {}).items():
                                    current_amount = self.ai_interface.completed_actions[-1]["response"][0].get(resource, 0) if self.ai_interface.completed_actions and self.ai_interface.completed_actions[-1]["action_type"] == interface.action_type.look and self.ai_interface.completed_actions[-1]["response"] and self.ai_interface.completed_actions[-1]["response"][0] else 0
                                    if current_amount > required_amount:
                                        for _ in range(current_amount - required_amount):
                                            self.ai_interface.send_set_object_down_action_and_block_until_response(resource)
                        else:
                            self.ai_interface.send_broadcast_action_and_block_until_response(f"wawa_{self.ai_interface.level}")
                            if self.elevation_broadcast_direction:
                                self.ai_interface.move_towards_direction(self.elevation_broadcast_direction)

                if (self.ai_interface.completed_actions 
                and self.ai_interface.completed_actions[-1]["action_type"] == interface.action_type.start_incantation 
                and self.ai_interface.completed_actions[-1]["response"]
                and self.ai_interface.completed_actions[-1]["response"] != "ko"):
                    self.state = WinnerAgent.State.none

