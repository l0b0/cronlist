PREFIX = /usr/local/bin

SCRIPT = $(notdir $(CURDIR))

$(SCRIPT): $(SCRIPT).c
	gcc -Wall -o $@ $<

install:
	install $(SCRIPT) $(PREFIX)

clean:
	-rm $(SCRIPT)
