prefix = /usr/local
exec_prefix ?= $(prefix)
bindir ?= $(exec_prefix)/bin
sysconfdir ?= $(prefix)/etc

script = $(notdir $(CURDIR))

$(script): $(script).c
	gcc -Wall -o $@ $<

install: $(script)
	install -d $(DESTDIR)$(bindir) $(DESTDIR)$(sysconfdir)/bash_completion.d
	install $(script) $(DESTDIR)$(bindir)
	install --mode 644 etc/bash_completion.d/$(script) $(DESTDIR)$(sysconfdir)/bash_completion.d

clean:
	-rm $(script)

include make-includes/variables.mk
