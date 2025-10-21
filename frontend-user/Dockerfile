ARG NODE_VERSION="22.21.0"
ARG BUN_VERSION="bun-v1.3.0"

FROM node:$NODE_VERSION-alpine

RUN apk add --no-cache bash curl

SHELL ["/bin/bash", "-c"]

WORKDIR /frontend-user

RUN curl -fsSL https://bun.com/install | bash -s $BUN_VERSION

ENV PATH="/root/.bun/bin:${PATH}"

COPY package.json bun.lock ./

RUN bun install --frozen-lockfile

EXPOSE 3000

CMD ["tail", "-f", "/dev/null"]
