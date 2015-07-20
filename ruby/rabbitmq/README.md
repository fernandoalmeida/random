# Testing RabbitMQ messaging system

## Server configuration with Docker container

    make build
    make run

#### Bunny

    cd bunny
    ./bunny.rb

#### Sneakers

    cd sneakers
    sneakers work Worker --require sneakers.rb | grep Hello

## Links

* [AMQP Concepts](https://www.rabbitmq.com/tutorials/amqp-concepts.html)
* [AMQP Protocol Specification - PDF](https://www.rabbitmq.com/resources/specs/amqp0-9-1.pdf)
* [RabbitMQ Management Command Line Interface](https://www.rabbitmq.com/management-cli.html)
* [Bunny - A popular, easy to use, well-maintained Ruby client for RabbitMQ](https://github.com/ruby-amqp/bunny)
* [Getting Started with Ruby and RabbitMQ with Bunny](http://rubybunny.info/articles/getting_started.html)
* [Sneakers - A fast background processing framework for Ruby and RabbitMQ](https://github.com/jondot/sneakers)
