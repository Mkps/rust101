SRC_NAME =	main.c acpi_access.c battery_management.c pwr_management.c

SRC = $(addprefix $(SRCDIR), $(SRC_NAME))
OBJ = target/

CC = cc

debug: CFLAGS = -g3 -pipe -Wall -Wextra -Werror -pedantic `pkg-config --cflags gtk4` -I$(INCDIR)
$(NAME): CFLAGS = -O2 `pkg-config --cflags gtk4` -I$(INCDIR)

LDFLAGS = `pkg-config --libs gtk4`

all: build

build:
	@echo "Building crate..."
	@cargo build
	@echo "Done!"

run:
	@echo "Running crate..."
	@cargo run

clean:
	@echo "Cleaning crate files..."
	@cargo clean

re: clean all

.PHONY: all clean re run build
