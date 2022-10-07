CLANG ?= clang
LLC ?= llc

.PHONY: all
all: fork.bpf.o

fork.bpf.o: fork.bpf.c
	$(CLANG) -O2 -target bpf -emit-llvm -c -g $< -o -| $(LLC) -march=bpf -filetype=obj -o $@
