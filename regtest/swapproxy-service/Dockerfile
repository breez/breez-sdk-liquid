FROM golang:1.23-alpine AS builder

WORKDIR /app
RUN apk add --no-cache gcc musl-dev
COPY swapproxy/go.mod swapproxy/go.sum ./
RUN go mod download
COPY swapproxy/ .

# Restore WebSocket origin acceptance for browser-based WASM tests.
RUN grep -q 'websocket.Accept(w, r, nil)' websocket.go && \
    sed -i 's|conn, err := websocket.Accept(w, r, nil)|options := \&websocket.AcceptOptions{OriginPatterns: []string{"*"}}\n\tconn, err := websocket.Accept(w, r, options)|' websocket.go

RUN CGO_ENABLED=1 GOOS=linux go build -o swapproxy

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/swapproxy .

EXPOSE 8387

CMD ["./swapproxy"] 