SUBDIRS=arch

# Config in project dir
include ../../make.conf

SRCS=$(wildcard *.c)
OBJS=$(SRCS:.c=.o)

INCL=

all: $(OBJS)

%.o: %.c
	$(CC) $(CFLAGS) $(INCL) -c $< -o $@
	

$(MAKECMDGOALS): $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@ $(MAKECMDGOALS)
	
clean:
	rm *.o
	
.PHONY: $(TOPTARGETS) $(SUBDIRS)
