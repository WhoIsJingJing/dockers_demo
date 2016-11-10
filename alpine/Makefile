
tag?=xiaoming/alpine/rust

all: run

build:
	@docker build -t $(tag) .

remove_image:
	@docker rmi -f $(tag)

list_img:
	docker images

run:
	docker run  -it $(tag)
