FROM rust:1.51

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        locales clang \
    && locale-gen ja-JP.UTF-8 \
    && localedef -f UTF-8 -i ja_JP ja_JP \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Asia/Tokyo \
    LANG=ja_JP.UTF-8 \
    LC_ALL=ja_JP.UTF-8 \
    APP_HOME=/usr/src/app

RUN ln -snf /usr/share/zoneinfo/${TZ} /etc/localtime \
    && echo ${TZ} > /etc/timezone

WORKDIR ${APP_HOME}

CMD ["cargo", "run", "--"]
