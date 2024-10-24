#!/usr/bin/env sh

set -e

# clippy

cmd="cargo cl --no-default-features"; echo $cmd; $cmd
cmd="cargo cl"; echo $cmd; $cmd
cmd="cargo cl -F std,freetype"; echo $cmd; $cmd

# tests

cmd="cargo t --no-default-features"; echo $cmd; $cmd
cmd="cargo t -F alloc"; echo $cmd; $cmd
cmd="cargo t -F alloc,freetype"; echo $cmd; $cmd

cmd="cargo t -F std,freetype"; echo $cmd; $cmd
cmd="cargo t -F std"; echo $cmd; $cmd

cmd="cargo t -F no_std"; echo $cmd; $cmd
cmd="cargo t -F no_std,freetype"; echo $cmd; $cmd

# doc

cmd="cargo +nightly doc -F _docsrs" echo $cmd; $cmd
