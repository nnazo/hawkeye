up:
	docker-compose --env-file .env up -d --build

down:
	docker-compose down -t 0

build:
	./minikube-build.sh

install: build
	helm install -n hawkeye hawkeye chart --set version=$(VERSION) --set secrets.NATALIE_WEBHOOK_URL=$(NATALIE_WEBHOOK_URL)

uninstall:
	helm uninstall -n hawkeye hawkeye

bootstrap:
	minikube addons enable default-storageclass
	minikube addons enable storage-provisioner
	kubectl create namespace hawkeye
