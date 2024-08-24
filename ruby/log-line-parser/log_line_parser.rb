class LogLineParser
  def initialize(line)
    @line = line
  end

  def message = @line.gsub(/^\[[A-Z]+\]:\s/, '').strip
  def log_level = @line.slice(/^\[([A-Z]+)\]/, 1).downcase!
  def reformat = "#{message} (#{log_level})"
end
