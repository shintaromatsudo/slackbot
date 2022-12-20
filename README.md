# Slackbot

## EC2環境構築
```
sudo yum update -y
sudo yum install -y gcc build-essential pkg-config libssl-dev
sudo amazon-linux-extras install -y docker
sudo systemctl start docker

sudo amazon-linux-extras install nginx1
sudo vi /etc/nginx/nginx.conf
location / {
    proxy_pass http://localhost:8080;
}
sudo systemctl restart nginx

sudo curl -L "https://github.com/docker/compose/releases/download/1.26.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
sudo ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose

scp -i "~/.ssh/shintaromatsudo.pem" -r ./Dockerfile ec2-user@ec2-12-345-678-90.ap-northeast-1.compute.amazonaws.com:~/slackbot/
scp -i "~/.ssh/shintaromatsudo.pem" -r ./docker-compose.yml ec2-user@ec2-12-345-678-90.ap-northeast-1.compute.amazonaws.com:~/slackbot/
scp -i "~/.ssh/shintaromatsudo.pem" -r ./Cargo.toml ec2-user@ec2-12-345-678-90.ap-northeast-1.compute.amazonaws.com:~/slackbot/
scp -i "~/.ssh/shintaromatsudo.pem" -r ./.env ec2-user@ec2-12-345-678-90.ap-northeast-1.compute.amazonaws.com:~/slackbot/
scp -i "~/.ssh/shintaromatsudo.pem" -r ./src ec2-user@ec2-12-345-678-90.ap-northeast-1.compute.amazonaws.com:~/slackbot/
scp -i "~/.ssh/shintaromatsudo.pem" -r ./api ec2-user@ec2-12-345-678-90.ap-northeast-1.compute.amazonaws.com:~/slackbot/
scp -i "~/.ssh/shintaromatsudo.pem" -r ./entity ec2-user@ec2-12-345-678-90.ap-northeast-1.compute.amazonaws.com:~/slackbot/
scp -i "~/.ssh/shintaromatsudo.pem" -r ./migration ec2-user@ec2-12-345-678-90.ap-northeast-1.compute.amazonaws.com:~/slackbot/

docker compose build
docker compose up

docker exec -it slackbot-api-1 bash
cargo install sea-orm-cli
sea-orm-cli migrate up

docker exec -it slackbot-db-1 bash
psql -U postgres

<!-- docker build ./ -t slackbot -->
<!-- docker run -it --name slackbot -p 8080:8080 slackbot -->
```
