class Bucket
	attr_reader :size, :name, :water
	def initialize(name, size)
		@name = name
		@size = size
		@water = 0
	end

	def empty? = @water == 0
	def full? = @water == @size
	def spilling? = @water > @size

	def empty!; @water = 0 end
	def pour_into!(bucket); @water = bucket.fill!(@water) end

	def fill!(amount=@size)
		@water += amount
		return 0 unless spilling?
		left_over, @water = @water - @size, @size
		left_over
	end
end

class TwoBucket
	attr_reader :moves
	def initialize(b1, b2, target, starting)
		@buckets = [Bucket.new("one", b1), Bucket.new("two", b2)]
		@target, @moves = target, 0
		@buckets.reverse! if starting == "two"
		until self.hit_target?
			self.pour!()
			@moves += 1
		end
	end

	def goal_bucket = @buckets.find{|b| b.water == @target}.name
	def other_bucket = @buckets.find{|b| b.water != @target}.water
	def hit_target? = @buckets.any?{|b| b.water == @target}

	def pour!()
		b1, b2 = @buckets
		case
		when b1.empty? then b1.fill!
		when b2.size == @target then b2.fill!
		when !b2.full? then b1.pour_into!(b2)
		else b2.empty!
		end
	end
end
