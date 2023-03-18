# Create an empty Docker image
FROM scratch

COPY /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# Copy binary from the local directory
COPY --chmod=755 ./exec /project

ENTRYPOINT ["/project"]
