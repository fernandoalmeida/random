class Producer
  attr_reader :connection, :queue_name

  def initialize(connection)
    @connection = connection
  end

  def publish(message, queue)
    exchange.publish(message, routing_key: queue)
    puts "Sent message: #{message}, queue: #{queue}"
  end

  private

  def exchange
    @exchange ||= channel.default_exchange
  end

  def channel
    @channel ||= connection.create_channel
  end
end
