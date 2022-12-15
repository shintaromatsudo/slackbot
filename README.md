# slackbot

## 開発環境構築
cp .env.example .env

cargo run

docker build ./ -t slackbot
docker run -itd --name slackbot -p 8080:8080 slackbot

## EC2環境構築
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
export PATH="$HOME/.cargo/bin:$PATH"

cargo new slackbot --bin
cd slackbot

sudo yum update -y
sudo amazon-linux-extras install -y docker
sudo systemctl start docker
sudo yum install -y gcc build-essential pkg-config libssl-dev

sudo amazon-linux-extras install nginx1
sudo vi /etc/nginx/nginx.conf
location / {
    proxy_pass http://localhost:8080;
}
sudo systemctl restart nginx

sudo curl -L "https://github.com/docker/compose/releases/download/1.26.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
sudo ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose

scp -i "~/.ssh/shintaromatsudo.pem" -r ./Dockerfile ec2-user@ec2-18-181-208-35.ap-northeast-1.compute.amazonaws.com:~/slackbot/
scp -i "~/.ssh/shintaromatsudo.pem" -r ./docker-compose.yml ec2-user@ec2-18-181-208-35.ap-northeast-1.compute.amazonaws.com:~/slackbot/
scp -i "~/.ssh/shintaromatsudo.pem" -r ./Cargo.toml ec2-user@ec2-18-181-208-35.ap-northeast-1.compute.amazonaws.com:~/slackbot/ 
scp -i "~/.ssh/shintaromatsudo.pem" -r ./.env ec2-user@ec2-18-181-208-35.ap-northeast-1.compute.amazonaws.com:~/slackbot/ 
scp -i "~/.ssh/shintaromatsudo.pem" -r ./src ec2-user@ec2-18-181-208-35.ap-northeast-1.compute.amazonaws.com:~/slackbot/ 
scp -i "~/.ssh/shintaromatsudo.pem" -r ./api ec2-user@ec2-18-181-208-35.ap-northeast-1.compute.amazonaws.com:~/slackbot/ 

sudo docker build ./ -t slackbot
sudo docker run -itd --name slackbot -p 8080:8080 slackbot
sudo docker-compose up --build
```
