SUBDIRS=$(wildcard */.)

# Config in project dir
include ../make.conf

all: $(SUBDIRS)

$(MAKECMDGOALS): $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@ $(MAKECMDGOALS)
	
clean:
	rm *.o
	
.PHONY: $(TOPTARGETS) $(SUBDIRS)
