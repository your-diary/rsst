#!/usr/bin/env python3

import json

import tweepy

json_string: str = None
with open('./conf/config.json') as f:
    json_string = f.read()

config: dict = json.loads(json_string)['triggers']['twitter']

consumer_key: str = config['consumer_key']
consumer_secret: str = config['consumer_secret']
access_token: str = config['access_token']
access_token_secret: str = config['access_token_secret']

client: tweepy.Client = tweepy.Client(
    consumer_key=consumer_key,
    consumer_secret=consumer_secret,
    access_token=access_token,
    access_token_secret=access_token_secret
)

text_list: list[str] = []
while (True):
    try:
        text_list.append(input())
    except EOFError:
        break
text: str = '\n'.join(text_list)

client.create_tweet(text=text)
