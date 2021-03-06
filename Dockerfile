FROM rustlang/rust:nightly

RUN apt-get update && apt-get install -y coinor-cbc coinor-libcbc-dev

WORKDIR /api
COPY . /api

EXPOSE 8080

ENTRYPOINT ["/api/docker-entrypoint.sh"]