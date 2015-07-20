# Docker helpers
build:
	docker build -t fernando/rabbitmq .

run:
	docker run \
	-v ${PWD}:/source \
	-w /source \
	-it fernando/rabbitmq /bin/bash

.PHONY: build run
