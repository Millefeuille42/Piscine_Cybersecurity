FROM alpine:3.17

RUN apk upgrade \
 && apk add nginx tor openssh

RUN adduser -D ft_onion && passwd -u ft_onion
RUN echo 'ft_onion:ft_onion' | chpasswd

RUN adduser -DH tord

RUN mkdir -p /etc/tor /var/tor/lib/hidden/webserv /var/tor/log /var/nginx/log /var/nginx/run /default
RUN chown tord:tord -R /etc/tor /etc/nginx /var/tor /var/nginx /default /var/lib/nginx
RUN chmod 700 -R /etc/tor /etc/nginx /var/tor /var/nginx /default /var/lib/nginx 
RUN rm /etc/nginx/nginx.conf

COPY --chown=tord:tord ./src/torrc /default/torrc
COPY --chown=tord:tord ./src/nginx.conf /default/nginx.conf
COPY --chown=tord:tord ./src/index.html /default/index.html
COPY --chown=tord:tord ./src/init.sh /init.sh

WORKDIR /var/tor/lib/hidden/webserv
USER tord
RUN ssh-keygen -A

EXPOSE 9001 9050
VOLUME [ "/etc/tor", "/etc/ssh", "/etc/nginx", "/var/tor", "/var/nginx" ]

ENTRYPOINT sh /init.sh
