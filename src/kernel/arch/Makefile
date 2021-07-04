# Config in project dir
include ../../../make.conf

SUBDIRS=$(ARCH)

all: $(SUBDIRS)

$(MAKECMDGOALS): $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@ $(MAKECMDGOALS)
	
.PHONY: $(TOPTARGETS) $(SUBDIRS)