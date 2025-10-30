#bin/bash
set -e

git add .
git commit -m "publish"
git push -u origin main
ssh quinn@serverus "cd /home/quinn/projects/civilian/api; git fetch; git pull; DOCKER_BUILDKIT=1 docker build . -t civilian-api; cd ..; docker save civilian-api -o builds/civilian-api.tar; scp builds/civilian-api.tar root@100.87.34.34:/root/docker/nginx/sites/civilian/builds"
ssh root@qcloud-1 "cd /root/docker/nginx/sites/civilian/builds && docker load -i civilian-api.tar && docker stop civilian-api-deploy && docker rm civilian-api-deploy && docker docker run --name civilian-api-deploy --restart=always -d --network \"host\" civilian-api"
