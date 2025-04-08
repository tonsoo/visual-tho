FROM rust:alpine3.21

RUN apk update && apk upgrade && apk add sudo

RUN adduser -D -u 1000 visualtho
USER visualtho

WORKDIR /app

CMD ["tail", "-f", "/dev/null"]
