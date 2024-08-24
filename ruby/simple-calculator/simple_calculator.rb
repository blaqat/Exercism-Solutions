class SimpleCalculator
  ALLOWED_OPERATIONS = ['+', '/', '*'].freeze
  class UnsupportedOperation < StandardError; end
  def self.calculate(first_operand, second_operand, operation)
  	raise UnsupportedOperation unless ALLOWED_OPERATIONS.include?(operation)
   	begin
	    answer = first_operand.send operation, second_operand
	   	"#{first_operand} #{operation} #{second_operand} = #{answer}"
    rescue ZeroDivisionError
	    "Division by zero is not allowed."
	  rescue => e
				raise ArgumentError.new(e.message)
	  end
  end
end
