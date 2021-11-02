#!/bin/bash 
build_user=green
workspace=$(builtin cd $PWD/../; pwd)
echo redeploy $workspace
su - $build_user -c "cd $workspace && git pull"
su - $build_user -c "cd $workspace && make build"
sudo systemctl restart chat