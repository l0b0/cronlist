prefix = /usr/local
exec_prefix ?= $(prefix)
bindir ?= $(exec_prefix)/bin
sysconfdir ?= $(prefix)/etc

SCRIPT = $(notdir $(CURDIR))

$(SCRIPT): $(SCRIPT).c
	gcc -Wall -o $@ $<

install: $(SCRIPT)
	install -d $(DESTDIR)$(bindir) $(DESTDIR)$(sysconfdir)/bash_completion.d
	install $(SCRIPT) $(DESTDIR)$(bindir)
	install --mode 644 etc/bash_completion.d/$(SCRIPT) $(DESTDIR)$(sysconfdir)/bash_completion.d

clean:
	-rm $(SCRIPT)

include make-includes/variables.mk
