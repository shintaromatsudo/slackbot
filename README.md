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

sudo apt-get update
sudo apt-get install -y gcc build-essential pkg-config libssl-dev
sudo apt install nginx
sudo vi /etc/nginx/sites-available/default
sudo systemctl restart nginx
location / {
                proxy_pass http://localhost:8080;
        }

sudo snap install docker
sudo docker build ./ -t slackbot
sudo docker run -itd --name slackbot -p 8080:8080 slackbot
```
