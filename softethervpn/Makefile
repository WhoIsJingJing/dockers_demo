
D_TAG?=siomiz/softethervpn
SHARE_PWD?=123456
A_USERS?=test:123456


all: run

build:
	@docker build -t $(D_TAG) .

remove_image:
	@docker rmi -f $(D_TAG)

list_img:
	docker images

run:
	docker run -d -p 500:500/udp -p 4500:4500/udp -p 1701:1701/tcp -e PSK=$(SHARE_PWD) -e USERS=$(A_USERS) $(D_TAG)
