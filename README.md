# Zapfrites

# Usage

```
./zappy_server -p [port] -x [width] -y [height] -n [team_name1 team_name2 ...] -c [clientsNb] -f [freq]
```

```
./zappy_gui -p [port] -h [hostname]
```

```
./zappy_ai -p [port] -h [hostname] -n [team_name]
```

## Server

//TODO

## Gui

//TODO

## AI

Different AI implementations have been created in order to test out features and try out different strategies.  
When starting an appy_ai intsance, you can specify the agent by using the `-a` flag followed by the agent name.  
Here are the available agents:
- winner (default): ai agent made to win the game in most situations  
//TODO

AIs where implemented using a 'interface' class that encapsulates the basic network logic and provides an API for the AI agents to interact with the game server.  
You can add new AI agents by creating a new agent class similar to the existing ones in the `ai/src/agents` directory and implementing your own logic for the agent's behavior.  