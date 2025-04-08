FROM rust:1.77-slim

RUN apt-get update && apt-get install -y \
    build-essential \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    curl \
    git \
    bash \
    libssl-dev \
    pkg-config \
    nodejs \
    npm \
    && apt-get clean

WORKDIR /app

CMD ["tail", "-f", "/dev/null"]
