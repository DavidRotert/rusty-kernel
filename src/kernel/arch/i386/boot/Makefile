# Config in project dir
include ../../../../../make.conf

SRCS=$(wildcard *.asm)
OBJS=$(SRCS:.asm=.o)

all: $(OBJS)

%.o: %.asm
	$(AS) $(ASFLAGS) $< -o $@
	
clean:
	rm *.o