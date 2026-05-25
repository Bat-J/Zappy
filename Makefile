SERVER_BIN 	:= zappy_server
GUI_BIN		:= zappy_gui
AI_BIN		:= zappy_ai

SHELL := /bin/bash

all: server ai

clean: clean_serv clean_ai

fclean: clean
	@echo "Removing binaries..."
	@rm -f $(SERVER_BIN) $(GUI_BIN) $(AI_BIN)

re: fclean all



server: $(SERVER_BIN)

$(SERVER_BIN):
	@echo "Building server..."
	@cargo build --release --manifest-path ./server/Cargo.toml
	@cp ./server/target/release/$(SERVER_BIN) ./

clean_serv:
	@echo "Cleaning server..."
	@cargo clean --manifest-path ./server/Cargo.toml



ai: $(AI_BIN)

$(AI_BIN):
	@echo "Building AI..."
	@if [ ! -d "./ai/.venv" ]; then \
		python3 -m venv ./ai/.venv; \
	fi
	@source ./ai/.venv/bin/activate && \
	(pip install -r ./ai/requirements.txt) > ai/venv_install.log 2>&1
	@cp ./ai/entry.sh ./$(AI_BIN)
	@chmod +x ./$(AI_BIN)

clean_ai:
	@echo "Cleaning AI..."
	@rm -f $(AI_BIN)
