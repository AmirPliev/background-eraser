FROM node:lts-alpine3.19 as frontend-build

WORKDIR /app

#all files needed for the build
COPY frontend/package.json .
COPY frontend/package-lock.json .
COPY frontend/tsconfig.json .
COPY frontend/svelte.config.js .
COPY frontend/vite.config.ts .

#all folders needed for the build
COPY frontend/ .

RUN npm i

RUN npm run build

FROM rust as backend-build
WORKDIR /app

COPY backend/ ./
RUN cargo build

FROM ubuntu:mantic-20240216 as production

WORKDIR /app
COPY --from=frontend-build /app/build /app/build
COPY --from=frontend-build /app/package-lock.json /app/package-lock.json
COPY --from=frontend-build /app/package.json /app/package.json

COPY --from=backend-build /app/target/debug/* /app/backend/
COPY backend/yolov8m-seg.onnx yolov8m-seg.onnx

ENV NODE_ENV=production
ENV LD_LIBRARY_PATH=/app/backend/


ENV NODE_VERSION=21.7.1
RUN apt-get update && apt-get install -y curl
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"

RUN export NVM_DIR="$HOME/.nvm" 
RUN [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" 
RUN npm install

ENV PORT=8080
EXPOSE 8080

COPY start.sh start.sh

ENTRYPOINT ["/bin/bash"]
CMD ["start.sh"]

