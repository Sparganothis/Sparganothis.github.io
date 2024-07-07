#!/bin/bash
set -e

git fetch -a --tags
git log --graph --all --decorate --oneline --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset'
