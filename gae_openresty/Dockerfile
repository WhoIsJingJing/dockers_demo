FROM openresty/openresty

MAINTAINER MingjunYang <yangaust@gmail.com>

RUN mkdir -p /var/log/app_engine
RUN mkdir -p /usr/local/openresty/nginx/www/_ah && \
    echo "healthy" > /usr/local/openresty/nginx/www/_ah/health

# Finally, all static assets.
ADD www/ /usr/local/openresty/nginx/www/
RUN chmod -R a+r /usr/local/openresty/nginx/www

ADD nginx/conf /usr/local/openresty/nginx/conf
ADD nginx/lualib /usr/local/openresty/nginx/lualib
ADD nginx/luasrc /usr/local/openresty/nginx/luasrc


# TODO: remove any other apt packages?
ENTRYPOINT ["/usr/local/openresty/bin/openresty", "-g", "daemon off;"]
