# Development guide - Bindings crate
This crate is responsible for building UniFFI bindings.

## Prerequisites
To build you need to first install:
- [Protobuf](https://protobuf.dev/installation/)
```bash
brew install protobuf
```

Set the ANDROID_NDK_HOME env variable to your Android NDK directory:
```bash
export ANDROID_NDK_HOME=<your android ndk directory>
```

On first usage you will need to run:
```bash
make init
```

## Building
To build bindings for individual languages please see the available [Makefile tasks](lib/bindings/makefile). Otherwise to build all bindings run:
```bash
make all
```

## Testing
```bash
make test
```
