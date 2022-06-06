all : release

DIR := $(CURDIR)

release :
	cargo build --release --package server

debug :
	cargo build --package server

image : release
	docker build -f docker/Dockerfile -t dream-proxima:latest .

clean :
	-rm -f ./target