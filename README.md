# rsst

# 1. About

Supports RSS 2.0 feeds and Atom feeds.

Written in Rust and Go.

Cross-platform.

CUI.

docker

# 2. Triggers

# 3. Configurations

# 4. Build

# 5. References

- [*RSS 2.0 specification - W3C*](https://validator.w3.org/feed/docs/rss2.html)

- [*Introduction to Atom - W3C*](https://validator.w3.org/feed/docs/atom.html)

# 6. Database Design

| Name | Description |
|:--|:--|
| `feeds` | Represents each site. |
| `feed_items` | Represents each blog entry. |

![](./readme_assets/database.png)

# 7. Algorithms

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