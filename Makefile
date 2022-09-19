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

psql:
	kubectl -n hawkeye exec -it $$(kubectl -n hawkeye get pods | grep hawkeyedb-0 | awk '{ print $$1 }') -- psql -U $(POSTGRES_USER) -d $(POSTGRES_DB)

release:
	git commit -m "Release $(VERSION)"
	git tag -a $(VERSION) -m "Release $(VERSION)" HEAD
	git push --follow-tags
