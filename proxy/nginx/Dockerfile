FROM nginx AS base
RUN rm /etc/nginx/conf.d/default.conf

FROM base AS dev
COPY dev.conf /etc/nginx/conf.d/default.conf

FROM base AS prod
COPY prod.conf /etc/nginx/conf.d/default.conf