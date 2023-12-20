FROM redis AS base
# RUN rm /usr/local/etc/redis/redis.conf

FROM base AS dev
COPY dev.conf /usr/local/etc/redis/redis.conf
CMD [ "redis-server", "/usr/local/etc/redis/redis.conf" ]

FROM base AS prod
COPY prod.conf /usr/local/etc/redis/redis.conf
CMD [ "redis-server", "/usr/local/etc/redis/redis.conf" ]
