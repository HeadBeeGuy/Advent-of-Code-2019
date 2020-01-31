# Advent of Code Day 14 part 1
# Non-working. I'm struggling to develop an algorithmic approach for this
# problem. Probably best to set it aside for now!

ore_ratios = {}

class Reaction
  attr_accessor :produced_with_ore, :reagents, :produced_count

  def initialize(reagents, product_chemical, produced_count)
    @product_chemical = product_chemical
    @produced_count = produced_count.to_i
    @produced_with_ore = false
    @reagents = []

    reagents.each do |reagent|
      # puts "I'm checking reagents! Here's one: #{reagent.inspect}"
      @reagents.push([reagent[0].to_i, reagent[1].to_sym])
      if reagent[1] == "ORE"
        @produced_with_ore = true
      end
    end
  end
end

Requirement = Struct.new(:count, :chemical)

reactions = {}
production_queue = []
ore_orders = {}

File.readlines("test3.txt").each do |line|
  reagents = line.scan(/([0-9]+) ([A-Z]+)/)
  if reagents[0][1] == "ORE"
    ore_ratios.merge!( { reagents[1][1].to_sym => { chemical: reagents[1][0].to_i, ore: reagents[0][0].to_i } } )
  end
  final_product = reagents.pop

  new_reaction = Reaction.new(reagents, final_product[1], final_product[0])
  reactions.merge!(final_product[1].to_sym => new_reaction)
end

# puts reactions.inspect


production_queue.push([1, :FUEL])

until production_queue.empty?
  make_this = production_queue.pop
  # puts "I'm being asked to make this: #{make_this}"
  requested_amount = make_this[0]
  chemical = make_this[1]
  if reactions[chemical].produced_with_ore
    # puts "I can make this with ore!"
    if ore_orders[chemical]
      ore_orders[chemical] += make_this[0]
    else
      ore_orders[chemical] = make_this[0]
    end
  else
    # puts "I can't make this with ore. Instead, the reagents are: #{reactions[chemical].reagents.inspect}"
    relevant_reaction = reactions[chemical]
    fulfilled = 0
    until fulfilled >= requested_amount
      relevant_reaction.reagents.each do |reagent|
        production_queue.push(reagent)
      end
      fulfilled += relevant_reaction.produced_count
    end
  end

  # puts "The production queue is now: #{production_queue.inspect}"
end

puts "I need to make these with ore: #{ore_orders.inspect}"
puts "Ore ratios are: #{ore_ratios.inspect}"

final_ore_required = 0

ore_orders.each do |element, quantity|
  puts "I'm being asked to make #{quantity} of #{element}."
  fulfilled = 0
  until fulfilled >= quantity
    final_ore_required += ore_ratios[element][:ore]
    fulfilled += ore_ratios[element][:chemical]
  end
end

puts "Producing 1 FUEL will require #{final_ore_required} units of ORE."
