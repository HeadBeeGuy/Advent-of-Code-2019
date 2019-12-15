# Advent of Code Day 6 part 2
# This one was pretty quick and dirty. I just look for a common ancestor and
# see how many steps it takes both nodes to reach them. There has to be a more
# elegant way to do this, but it runs quickly so I'll call it good and move on!

bodies = {} # structure: { child_name: parent_name }
all_bodies = []

# construct the tree
File.readlines("../../input.txt").each do |line|
  # I'll assume input is well-formed and contains 2 elements.
  relationship = line.scan(/[A-Z0-9]{1,3}/)
  # item 0: parent
  # item 1: child
  bodies[ relationship[1] ] = relationship[0]
  unless all_bodies.include? relationship[0]
    all_bodies << relationship[0]
  end
  unless all_bodies.include? relationship[1]
    all_bodies << relationship[1]
  end
end

# trace route from YOU to root node
you_route = []
you_parent = bodies["YOU"]
until you_parent.nil?
  you_route << you_parent
  you_parent = bodies[you_parent]
end

# iterate on SAN route until it reaches a node that YOU traversed
santa_parent = bodies["SAN"]
san_count = 0
common_node = nil
until common_node
  if you_route.include? santa_parent
    common_node = santa_parent
  else
    santa_parent = bodies[santa_parent]
    san_count += 1
    if santa_parent.nil?
      puts "There wasn't a common parent! Control+C the program because it's stuck!"
    end
  end
end

# traverse the YOU route again - inefficient, but it makes the code easy to write
you_count = 0
you_node = bodies["YOU"]
until you_node == common_node
  you_count += 1
  you_node = bodies[you_node]
end

puts "Distance from you to common node: #{you_count} and for Santa: #{san_count}"
puts "Combined distance: #{you_count + san_count}"
