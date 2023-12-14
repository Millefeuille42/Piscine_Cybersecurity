FROM alpine:3.17

RUN apk upgrade \
 && apk add nginx tor openssh

RUN ssh-keygen -A

RUN adduser -DH tord
RUN adduser -D ft_onion && passwd -u ft_onion && echo 'ft_onion:ft_onion' | chpasswd

RUN mkdir -p /etc/tor /var/tor/lib/hidden/webserv /var/tor/log /var/nginx/log /var/nginx/run /default
RUN chown tord:tord -R /var/tor /default
RUN chmod 700 -R /var/tor /default

# COPY --chown=tord:tord ./src/torrc /default/torrc
# COPY --chown=tord:tord ./src/nginx.conf /default/nginx.conf
# COPY --chown=tord:tord ./src/index.html /default/index.html
# COPY --chown=tord:tord ./src/sshd_config /default/sshd_config
# COPY --chown=tord:tord ./src/init.sh /init.sh
COPY --chown=tord:tord ./src /default
RUN cp /default/sshd_config /etc/ssh/sshd_config
RUN cp /default/nginx.conf /etc/nginx/nginx.conf

WORKDIR /var/tor/lib/hidden/webserv
EXPOSE 9001 9050

ENTRYPOINT sh /default/init.sh
