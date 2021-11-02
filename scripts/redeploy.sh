#!/bin/bash 
build_user=green
current_dir=$(pwd)
workspace=$(dirname "$current_dir")
echo $workspace
su - $build_user -c "cd $workspace && git pull"
su - $build_user -c "cd $workspace && make build"
sudo systemctl restart chat