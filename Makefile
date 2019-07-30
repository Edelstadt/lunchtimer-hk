build:
	docker build -t lunchtimer-hk .
	docker-compose up -d --force-recreate