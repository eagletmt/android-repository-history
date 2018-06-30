FROM golang:1.10-stretch as builder

RUN mkdir -p /build
COPY update.go /build/
WORKDIR /build
RUN go build update.go

FROM debian:stretch-slim
RUN apt update && apt install -y ca-certificates git
COPY --from=builder /build/update /root/
COPY auto.sh /root/
CMD ["/root/auto.sh"]
