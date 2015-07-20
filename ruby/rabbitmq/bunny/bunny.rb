#!/usr/bin/env ruby

require 'bunny'
require_relative 'app/consumer'
require_relative 'app/producer'

begin
  puts 'Press CTRL+C to exit'

  BUNNY_QUEUES = [
    HIGH_PRIORITY_QUEUE = 'bunny.queue.high',
    LOW_PRIORITY_QUEUE = 'bunny.queue.low'
  ]

  # RabbitMQ server TCP connection (shared)
  connection = Bunny.new
  connection.start

  # Consumer - subscribe to queues
  consumer1 = Consumer.new(connection, HIGH_PRIORITY_QUEUE).subscribe
  consumer2 = Consumer.new(connection, LOW_PRIORITY_QUEUE).subscribe

  # Producer - publish messages
  producer = Producer.new(connection)
  loop do
    message = rand.to_s
    queue = BUNNY_QUEUES.sample

    producer.publish(message, queue)

    sleep 0.5
  end
rescue Interrupt => _
  connection.close

  exit(0)
end
