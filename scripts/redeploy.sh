#!/bin/bash 
build_user=green
workspace="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )/../" &> /dev/null && pwd )"
echo redeploy $workspace
su - $build_user -c "cd $workspace && git pull"
su - $build_user -c "cd $workspace && make build"
sudo systemctl restart chat