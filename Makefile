PREFIX = /usr/local/bin

SCRIPT = $(notdir $(CURDIR))

$(SCRIPT): $(SCRIPT).c
	gcc -Wall -o $@ $<

install: $(SCRIPT)
	install $(SCRIPT) $(PREFIX)

clean:
	-rm $(SCRIPT)
