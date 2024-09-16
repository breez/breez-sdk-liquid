#!/bin/bash
cd ../..
make bindings-swift
rm -rf ../../packages/flutter/ios/bindings-swift
cp -r langs/swift ../../packages/flutter/ios/bindings-swift
rm -f ../../packages/flutter/ios/bindings-swift/Package.swift
