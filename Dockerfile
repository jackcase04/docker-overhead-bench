FROM debian:bookworm-slim
COPY target/release/server /app/server
COPY data/users.json /app/data/users.json
COPY data/transactions.json /app/data/transactions.json
WORKDIR /app
EXPOSE 7878
CMD ["chrt", "-f", "90", "./server"]