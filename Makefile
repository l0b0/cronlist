PREFIX = /usr/local/bin

SCRIPT = $(notdir $(CURDIR))

$(SCRIPT): $(SCRIPT).c
	gcc -Wall -o $@ $<

install: $(SCRIPT)
	install $(SCRIPT) $(PREFIX)
	install --mode 644 etc/bash_completion.d/$(SCRIPT) /etc/bash_completion.d/

clean:
	-rm $(SCRIPT)

include make-includes/variables.mk
