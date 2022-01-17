#!/bin/sh

cargo build --release
mv -i -v target/release/doit $HOME/.cargo/bin/
touch ~/tasks/tasks.json

echo "{\"tasks_lists\": {}}" > ~/tasks/tasks.json


