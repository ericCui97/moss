CC = gcc
CFLAGS = -Wall -Wextra -g

SRCS = main.c chunk.c memory.c debug.c
OBJS = $(SRCS:.c=.o)

TARGET = cmoss

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) $(TARGET)