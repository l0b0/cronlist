prefix = /usr/local
exec_prefix ?= $(prefix)
bindir ?= $(exec_prefix)/bin
sysconfdir ?= $(prefix)/etc

CC = gcc
CFLAGS = -Wall

name = $(notdir $(CURDIR))
exe = $(name).out

.PHONY: all
all: $(exe)

%.out: %.c
	$(CC) $(CFLAGS) -o $@ $<

.PHONY: install
install: $(exe)
	install -d $(DESTDIR)$(bindir) $(DESTDIR)$(sysconfdir)/bash_completion.d
	install $(exe) $(DESTDIR)$(bindir)/$(name)
	install --mode 644 etc/bash_completion.d/$(name) $(DESTDIR)$(sysconfdir)/bash_completion.d

.PHONY: clean
clean:
	-rm $(exe)

include make-includes/variables.mk
