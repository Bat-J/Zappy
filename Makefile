SERVER_BIN 	:= zappy_server
GUI_BIN		:= zappy_gui
AI_BIN		:= zappy_ai

all: $(SERVER_BIN)

$(SERVER_BIN):
	@cargo build --release --manifest-path ./server/Cargo.toml
	@cp ./server/target/release/$(SERVER_BIN) ./

$(GUI_BIN):
	@echo "Building GUI..."
	@cd gui && cmake -S . -B build && cmake --build build --config Release
	@cp ./gui/build/$(GUI_BIN) ./

clean_serv:
	@echo "Cleaning server..."
	@cargo clean --manifest-path ./server/Cargo.toml

clean: clean_serv

fclean: clean
	@echo "Removing binaries..."
	@rm -f $(SERVER_BIN) $(GUI_BIN) $(AI_BIN)

re: fclean all
