SRC_NAME =	main.c acpi_access.c battery_management.c pwr_management.c

SRC = $(addprefix $(SRCDIR), $(SRC_NAME))
OBJ = ./target

CC = cc

debug: CFLAGS = -g3 -pipe -Wall -Wextra -Werror -pedantic `pkg-config --cflags gtk4` -I$(INCDIR)
$(NAME): CFLAGS = -O2 `pkg-config --cflags gtk4` -I$(INCDIR)

LDFLAGS = `pkg-config --libs gtk4`

all: build

build:
	@cargo build
	@echo "Done!"

run:
	@echo "Cargo Run"
	@cargo run

clean:
	@echo "Cleaning object files..."
	@rm Cargo.lock
	@rm -rf $(OBJ_DIR)

re: fclean all

.PHONY: all clean re run
