import sys

for arg in sys.argv:
    if arg == "--help":
        from print_help import print_help
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

# connecting
## WIP POC code past this point

import networking

print(f"Connecting to {machine_name} on port {port_number} as team '{team_name}'...")

# conecting to the server
connection = networking.Connection(machine_name, port_number)
if not connection.connect():
    print("Error: Could not connect to the server.")
    sys.exit(84)

while connection.receive_message() != "WELCOME\n":
    pass

# send team name
connection.send_message(team_name + "\n")

team_number = int(connection.receive_message().strip())
print(f"Connected as team {team_number}.")

map_size_message = connection.receive_message()
map_x, map_y = map(int, map_size_message.strip().split())
print(f"Map size: {map_x}x{map_y}")