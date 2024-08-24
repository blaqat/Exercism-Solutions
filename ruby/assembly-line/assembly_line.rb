class AssemblyLine
	CARS_PER_HOUR = 221

  def initialize(speed)
    @speed = speed
  end

  def success_rate
		if @speed <= 4
			1.0
		elsif @speed <= 8
			0.9
		elsif @speed == 9
			0.8
		elsif @speed == 10
			0.77
		end
  end

  def production_rate_per_hour
  	@speed * CARS_PER_HOUR * success_rate
  end

  def working_items_per_minute
    production_rate_per_hour.to_i / 60
  end
end
