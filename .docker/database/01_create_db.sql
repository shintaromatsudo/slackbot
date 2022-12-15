CREATE ROLE slackbot_role WITH CREATEDB LOGIN PASSWORD 'slackbot_pass';
CREATE DATABASE slackbot WITH TEMPLATE = template0 OWNER = 'slackbot_role' ENCODING = 'UTF8' LC_COLLATE = 'C' LC_CTYPE = 'C';
