# rsst

# 1. About

*rsst* is a json-driven CUI rss/atom feed reader for command-line addictive users.

It is written in Rust.

Main Features:

- Cross-platform.

- Both RSS 2.0 and Atom are supported.

- `Dockerfile` and `docker-compose.yaml` are included.

- Configurations (including a list of feed sites) are written in a single JSON file.

- User-defined *triggers* (see below).

# 2. Triggers

When rsst detects new blog entries, it runs a series of registered *triggers* for each entry.

For example, `Discord` trigger is by default implemented, which sends a message to your Discord server.

|![](readme_assets/discord.png)|
|:--:|
| An example of `Discord` trigger. |

In a similar manner, any triggers can be implemented by yourself (or via feature requests perhaps). Here's a part of my future ideas:

- `Twitter` trigger to tweet an new blog entry using your private bot account.

- `Pocket` trigger to add an new blog entry to your Pocket list.

# 3. Configurations

rsst reads `./conf/config.json` as the configuration file. This file is auto-reloaded without the need of restarting a Docker container.

You need not to write this config file from scratch. Just start by copying the template:

```bash
cp conf/config_template.json conf/config.json
vi conf/config.json
```

Example:

```json
{
    "should_log_debug": true,
    "database_file": "./conf/db.sqlite3",
    "triggers": {
        "discord": {
            "enabled": true,
            "webhook_url": "https://discord.com/api/webhooks/217647058221348866/DPeZfHx66dN2OeHeTBmMWp347pIibEDmSCbz7k3HMh7aku9wKJqKaTjCyfJnMSKsnvsd"
        }
    },
    "feed_url_list": [
        "https://archlinux.org/feeds/news/",
        "https://alpinelinux.org/atom.xml",
        "https://blog.rust-lang.org/feed.xml",
        "https://go.dev/blog/feed.atom"
    ]
}
```

( :warning: The value of `webhook_url` in this example is invalid of course. Use yours.)

# 4. Build

1. First clone this repository and modify the configuration file as you like.

    ```bash
    git clone 'https://github.com/your-diary/rsst'
    cd rsst
    cp conf/config_template.json conf/config.json
    vi conf/config.json
    ```

2. Then build a docker image.

    ```bash
    docker compose build
    ```

3. (Optional) By default, rsst checks new entries for each registered site once an hour. You can customize this interval by changing the environment variable `${RSST_INTERVAL_MIN}`.

    ```bash
    vi docker-compose.yaml
    ```

4. Start a docker container.

    ```
    docker compose up -d
    ```

5. (Optional.) At this point, the host-side `./conf/` directory is bind-mounted to the docker container. A SQLite3 database and a log file are created in this directory. To see the real-time log output, run either the followings, both of which behave exactly the same.

    ```bash
    docker logs -f rsst
    ```

    ```bash
    tail -f conf/log.txt
    ```

# 5. For Developers

## 5.1 References

- [*RSS 2.0 specification - W3C*](https://validator.w3.org/feed/docs/rss2.html)

- [*Introduction to Atom - W3C*](https://validator.w3.org/feed/docs/atom.html)

## 5.2 Database Design

| Name | Description |
|:--|:--|
| `feeds` | Represents each site. |
| `feed_items` | Represents each blog entry. |

![](./readme_assets/database.png)

## 5.3 Algorithms

![](./readme_assets/plantuml.png)

<!--

```plantuml
@startuml

start

group read config file
    :url_list := a list of urls;
    :trigger_list := a list of triggers;
end group

:connect to database;

while (for url in url_list)
    :retrieve xml from url;
    :check feed type (Rss or Atom);
    if (new site?) then (yes)
        while (for trigger in triggers)
            :pull trigger for the latest feed entry\n(This is to confirm that triggers work for the new site.);
        endwhile
        if (all triggers succeeded?) then (yes)
            :insert site into database;
            :insert all entries into database;
        else (no)
        endif
    else (no)
        :A := select existent entries for the site from database;
        :B := newly retrieved entries;
        :C := B \\ A;
        while (for c in C)
            while (for trigger in triggers)
                :pull trigger for c;
            endwhile
            if (all triggers succeeded?) then (yes)
                :insert c into database;
            else (no)
            endif
        endwhile
    endif
endwhile

stop

@enduml
```

-->
