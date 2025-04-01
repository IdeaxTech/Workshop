.PHONY: all cpp rust go python ts java ruby dart kotlin clean

all: cpp rust go python ts java ruby dart kotlin

cpp:
	cd cpp && g++ -o janken janken.cpp && ./janken

rust:
	cd rust && cargo run

go:
	cd go && go run janken.go

python:
	cd python && python3 janken.py

ts:
	cd typescript && tsc janken.ts && node janken.js

java:
	cd java && javac Janken.java && java Janken

ruby:
	cd ruby && ruby janken.rb

dart:
	cd dart && dart pub add args && dart run janken.dart --choice 0

kotlin:
	cd kotlin && kotlinc Janken.kt -include-runtime -d Janken.jar && java -jar Janken.jar

clean:
	rm -f cpp/janken_cpp java/Janken.class typescript/janken.js
