class Consumer
  attr_reader :connection, :queue_name

  def initialize(connection, queue_name)
    @connection = connection
    @queue_name = queue_name
  end

  def subscribe
    queue.subscribe do |delivery_info, metadata, payload|
      puts "[#{queue_name}] Consumed: #{payload}"
    end
  end

  private

  def queue
    @queue ||= channel.queue(queue_name)
  end

  def channel
    @channel ||= connection.create_channel
  end
end
