#!/bin/sh
cargo leptos watch & npx tailwindcss -i ./style/tailwind.css -o ./style/output.css --watch && kill -9 $!
