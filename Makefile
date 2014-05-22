prefix = /usr/local
exec_prefix ?= $(prefix)
bindir ?= $(exec_prefix)/bin
sysconfdir ?= $(prefix)/etc

name = $(notdir $(CURDIR))

$(name): $(name).c
.PHONY: all
	gcc -Wall -o $@ $<

.PHONY: install
install: $(name)
	install -d $(DESTDIR)$(bindir) $(DESTDIR)$(sysconfdir)/bash_completion.d
	install $(name) $(DESTDIR)$(bindir)
	install --mode 644 etc/bash_completion.d/$(name) $(DESTDIR)$(sysconfdir)/bash_completion.d

.PHONY: clean
clean:
	-rm $(name)

include make-includes/variables.mk
