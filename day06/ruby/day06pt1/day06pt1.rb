# Advent of Code Day 6 part 1
#
# I decided to do this in Ruby to get it out of the way quickly.
# In abstract terms, it looks like all we need to do is plot out a tree
# structure, go through to every node and count how many nodes it takes to get
# to the top, then total that up for all nodes.

# I can just use hash maps for the names of each celestial body, and make their
# value the name of their parent. Then the parent can be the key for the next
# search to find its parent, and so on. We can only go up the tree, not down
# it, but that's all we need for this problem!

bodies = {} # structure: { child_name: parent_name }
all_bodies = []

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

total_parent_count = 0

all_bodies.each do |body|
  current_parent = bodies[body]
  until current_parent.nil?
    current_parent = bodies[current_parent]
    total_parent_count += 1
  end
end


puts "The total orbit count is #{total_parent_count}."
