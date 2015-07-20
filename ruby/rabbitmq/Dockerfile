FROM ruby:2.2.2
MAINTAINER Fernando Almeida

# Install server
RUN echo "deb http://www.rabbitmq.com/debian/ testing main" > \
         /etc/apt/sources.list.d/rabbitmq.list
RUN curl http://www.rabbitmq.com/rabbitmq-signing-key-public.asc | apt-key add -
RUN apt-get -qqy update
RUN apt-get -qy install rabbitmq-server

# Enable admin (plugin)
RUN rabbitmq-plugins enable rabbitmq_management
RUN /etc/init.d/rabbitmq-server start && \
    wget http://localhost:15672/cli/rabbitmqadmin -P /usr/local/bin/
RUN chmod +x /usr/local/bin/rabbitmqadmin
RUN rabbitmqadmin --bash-completion > /etc/bash_completion.d/rabbitmqadmin

# Install ruby gems (clients)
RUN gem install bunny sneakers

# Autostart service on user login
RUN echo "/etc/init.d/rabbitmq-server start" >> /etc/bash.bashrc
