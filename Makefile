# Config in project dir
include ./make.conf

SUBDIRS=$(SRCDIR)

KERNEL_OBJS+=$(wildcard $(SRCDIR)/kernel/arch/$(ARCH)/*/*.o)
KERNEL_OBJS+=$(TARGETDIR)/$(PROJECTNAME).a

KERNEL_BIN=$(TARGETDIR)/kernel.bin

all: link-kernel

link-kernel: c-build rust-build
	$(LN) $(LNFLAGS) $(KERNEL_OBJS) -T $(SRCDIR)/linker.ld -o $(KERNEL_BIN)

rust-build:
	cargo build

c-build:
	$(MAKE) -C $(SRCDIR) $(MAKECMDGOALS)

run:
	$(QEMU_EXEC) -kernel $(KERNEL_BIN)

clean-kernel-bin:
	rm -r $(KERNEL_BIN)

clean:
	cargo clean
	$(MAKE) -C $(SRCDIR) clean
