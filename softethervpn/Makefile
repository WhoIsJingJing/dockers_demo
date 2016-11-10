
D_TAG?=siomiz/softethervpn
SHARE_PWD?=123456
U_NAME?=test
PASSWORD?=123456


all: run

build:
	@docker build -t $(D_TAG) .

remove_image:
	@docker rmi -f $(D_TAG)

list_img:
	docker images

run:
	docker run -d -p 500:500/udp -p 4500:4500/udp -p 1701:1701/tcp -e PSK=$(SHARE_PWD) -e USERNAME=$(U_NAME) -e PASSWORD=$(PASSWORD) $(D_TAG)
