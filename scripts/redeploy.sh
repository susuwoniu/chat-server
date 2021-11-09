#!/bin/bash 
set -o allexport; source .env; set +o allexport
ip4=$(/sbin/ip -o -4 addr list eth0 | awk '{print $4}' | cut -d/ -f1)
build_user=green
workspace="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )/../" &> /dev/null && pwd )"
echo redeploy $workspace
su - $build_user -c "cd $workspace && git pull"
su - $build_user -c "cd $workspace && make build"
sudo systemctl restart chat
# send notice
curl -X POST -H "Content-Type: application/json" -d "{\"value1\":\"成功部署chat-server到${ip4}\"}" https://maker.ifttt.com/trigger/notice/with/key/$IFTTT_KEY