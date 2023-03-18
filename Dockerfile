FROM alpine:latest AS ca

RUN apk add --no-cache ca-certificates-bundle

# Create an empty Docker image
FROM scratch

ARG TARGETOS
ARG TARGETARCH

COPY --from=ca /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# Copy binary from the local directory
COPY --chmod=755 ./${TARGETOS}-${TARGETARCH} /project

ENTRYPOINT ["/project"]
