FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY ./rust_restful_api /bin/
COPY ./Rocket.toml .
EXPOSE 8000
ENTRYPOINT ["/bin/rust_restful_api"]
