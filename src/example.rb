threads = []

10.times do
  threads << Thread.new do
    count = 0

    5_000_000.times do
      count += 1
    end

    count
  end
end

threads.each { |thread| puts "Thread finished with count=#{thread.value}" }

puts "done!"
