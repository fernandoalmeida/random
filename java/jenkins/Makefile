build:
	docker build \
		-t fernandoalmeida/jenkins \
		.

run:
	docker run \
	  	-v ${PWD}/data:/app \
		-p 8080:8080 \
		-it fernandoalmeida/jenkins

clear:
	sudo rm -fr data
