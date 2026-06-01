import sys
from print_help import print_help
import interface
import agents.simple_lawn_mower

for arg in sys.argv:
    if arg == "--help":
        print_help()
        sys.exit(0)

port_number = None
team_name = None
machine_name = "localhost"

for i in range(len(sys.argv)):
    if sys.argv[i] == "-p" and i + 1 < len(sys.argv):
        port_number = int(sys.argv[i + 1])
    elif sys.argv[i] == "-n" and i + 1 < len(sys.argv):
        team_name = sys.argv[i + 1]
    elif sys.argv[i] == "-h" and i + 1 < len(sys.argv):
        machine_name = sys.argv[i + 1]

if port_number is None:
    print("Error: Port number is required.")
    sys.exit(84)
if team_name is None:
    print("Error: Team name is required.")
    sys.exit(84)

# conecting to the server

print(f"Connecting to {machine_name} on port {port_number} as team '{team_name}'...")

ai_interface = interface.AI_server_interface(machine_name, port_number, team_name)

if not ai_interface.connect():
    print("Failed to connect to the server.")
else:
    print("connected to the server")
    print(
        "number of unused slots: " + str(ai_interface.last_known_number_of_unused_slots)
    )
    print(
        "world dimensions: "
        + str(ai_interface.world_x)
        + " x "
        + str(ai_interface.world_y)
    )

# creating and running the agent

agent = agents.simple_lawn_mower.SimpleLawnMowerAgent(ai_interface)

agent.run()
