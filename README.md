# slackbot

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





curl -X POST -H "Authorization: Bearer xoxb-170884278241-4218562581984-edpsutUCT74L7GtNSYXUszaI" \
    -H "Content-type: application/json" \
    --data '{"text" : "Hello World", "channel" : "C03F8QRLYUC", "thread_ts": "1666197586.071719"}' \
    https://slack.com/api/chat.postMessage


curl -X POST -H "Content-Type: application/json" -d '{"event": {"channel": "D011Q70BHD3"}, "token": "7qKc3W5jMAhqg6N1Hrig5v5k"}' localhost:8080/slackbot


curl -X POST -H "Content-Type: application/json" -d '{"event": {"channel": "C03F8QRLYUC", "text": "happy", "thread_ts": "1666197586.071719", "user": "U046EGJH3UY"}, "token": "7qKc3W5jMAhqg6N1Hrig5v5k"}' localhost:8080/slackbot
