git add .
git commit -m "publish"
git push -u origin main
ssh quinn@serverus "cd /home/quinn/projects/civilian; git fetch; git pull; docker build . -t civilian-api; docker save civilian-api > builds/civilian-api.tar; scp builds/civilian-api.tar root@100.87.34.34:/root/docker/nginx/sites/civilian/builds"
