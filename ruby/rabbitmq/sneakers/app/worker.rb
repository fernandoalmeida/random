class Worker
  include Sneakers::Worker

  from_queue :sneakers
  attr_reader :entry

  def work(entry)
    @entry = entry

    puts "Consumed message: #{entry.inspect}"
  end

  private

  def attributes
    @attributes ||= JSON.parse(entry)
  end

  def message
    attributes[:message]
  end

  def type
    attributes[:type]
  end

  def error
    attributes[:error]
  end
end
