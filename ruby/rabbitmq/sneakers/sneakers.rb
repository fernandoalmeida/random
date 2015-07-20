#!/usr/bin/env ruby
require 'sneakers'
require_relative 'app/worker'

begin
  puts 'Press CTRL+C to exit'

  # RabbitMQ command line
  `rabbitmqadmin declare queue name=sneakers`

  puts 'Start publishing messages'

  10.times do |n|
    message = "Hello Sneakers #{n}"
    `rabbitmqadmin publish exchange=amq.default \
                           routing_key=sneakers \
                           payload="#{message}"`
    puts "Published message: #{message}"
  end

  puts 'Finish publishing messages'
rescue Interrupt => _
  exit(0)
end
