# FRONTEND

FROM rust:1 AS frontend-builder

WORKDIR /app/frontend

# Install trunk and wasm target
RUN curl -sSfL https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-musl.tar.gz | tar -xzf - -C /usr/local/bin/ \
&& rustup target add wasm32-unknown-unknown

# Copy manifests first for better layer caching
COPY todoapp_frontend/Cargo.* ./

# Now copy the full project
COPY todoapp_frontend ./
RUN echo "BACKEND_URL=/api/" > .env

# Build the Yew app
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    trunk build --release


# BACKEND

FROM rust:1-alpine AS backend-builder

RUN apk add --no-cache build-base

WORKDIR /app/backend
COPY todoapp_backend/Cargo.* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

RUN rm -Rf src target/release/todoapp_backend target/release/deps/todoapp_backend*

COPY todoapp_backend/ ./
RUN cargo build --release

# Nginx
FROM nginx:alpine
RUN apk add --no-cache libgcc libstdc++ gcompat
WORKDIR /app

ENV RUST_BACKTRACE=1
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV DATABASE_URL=/app/data/db.sqlite
ENV ROCKET_DATABASES="{sqlite={url=\"${DATABASE_URL}\"}}"

COPY --from=backend-builder /app/backend/target/release/todoapp_backend /app/backend_bin

COPY --from=frontend-builder /app/frontend/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY entrypoint.sh .
RUN chmod +x entrypoint.sh
RUN mkdir data

EXPOSE 80

CMD ["/app/entrypoint.sh"]