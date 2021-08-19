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
	cargo build $(CARGO_ARGS)

c-build:
	$(MAKE) -C $(SRCDIR) $(MAKECMDGOALS)

run:
	$(QEMU_EXEC) -kernel $(KERNEL_BIN)
	
run-gdb: run-qemu-gdb
	gdb -ex "target remote :1234" -q $(KERNEL_BIN)
	
run-qemu-gdb:
	$(QEMU_EXEC) -kernel $(KERNEL_BIN) -s -S &

clean-kernel-bin:
	rm -r $(KERNEL_BIN)

clean:
	cargo clean
	$(MAKE) -C $(SRCDIR) clean
