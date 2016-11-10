
INSTANCE?=gae_rust_demo
PORT_MAP?=8080:8080

default: run
	
build:
	docker build -t $(INSTANCE) .

remove_image:
	docker rmi -f $(INSTANCE)

images:
	docker images

run:
	docker run -it -p $(PORT_MAP) --rm  $(INSTANCE) -v $$(pwd):/source
