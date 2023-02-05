CLANG ?= clang
LLC ?= llc

OUT_DIR ?= dist

LIBBPF_SRC = libbpf/src
LIBBPF_DIR = $(OUT_DIR)/libbpf
LIBBPF_HEADERS := $(LIBBPF_DIR)/usr/include

.PHONY: all
all: $(OUT_DIR)/fork.bpf.o

$(OUT_DIR):
	mkdir -p $@

$(LIBBPF_HEADERS) $(LIBBPF_HEADERS)/bpf $(LIBBPF_HEADERS)/linux:
	$(MAKE) -C $(LIBBPF_SRC) DESTDIR=$(abspath $(LIBBPF_DIR)) \
		install_headers	install_uapi_headers

$(OUT_DIR)/fork.bpf.o: fork.bpf.c $(LIBBPF_HEADERS)
	$(CLANG) -I $(LIBBPF_HEADERS) -O2 -target bpf -emit-llvm -c -g $< -o -| \
		$(LLC) -march=bpf -filetype=obj -o $@
