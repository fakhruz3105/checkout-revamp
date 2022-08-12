#!/bin/bash

rm src/schema.rs
rm app.db

diesel setup
diesel migration run