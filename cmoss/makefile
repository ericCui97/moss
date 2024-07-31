CC = gcc
CFLAGS = -Wall -Wextra -g

SRCS = main.c chunk.c memory.c debug.c value.c vm.c compiler.c scanner.c object.c hashtable.c
OBJS = $(patsubst %.c,build/%.o,$(SRCS))

TARGET = cmoss

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJS)

build/%.o: %.c | build
	$(CC) $(CFLAGS) -c $< -o $@

build:
	mkdir -p build

clean:
	rm -rf build $(TARGET)