# Create an empty Docker image
FROM scratch

# Copy binary from the local directory
COPY --chmod=755 ./built /project

ENTRYPOINT ["/project"]
