# build bot docker file
sudo docker build --build-arg CACHE_DATE=$(date +%Y-%m-%d:%H:%M:%S) . -t ttb-bot -f production.Dockerfile

# run compose 
sudo docker compose -f docker-compose.prod.yml up -d
