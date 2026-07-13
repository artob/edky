abort("Expected Ruby 3.4+, but got #{RUBY_VERSION}.") if RUBY_VERSION < '3.4.0'

require 'csv'  # `gem install csv`

SAMPLES = CSV.table('samples.csv')
  .inject({}) { |hash, row| hash[row[:format].to_sym] = row[:sample]; hash }
  .freeze

task :default => 'samples:markdown'

namespace :samples do
  desc "Output the samples table for the README"
  task markdown: %w[samples.csv] do |t|
    puts "| Feature         | Example Public Key |"
    puts "| :-------------- | :----------------- |"
    SAMPLES.each do |format, sample|
      quoted_format = "`#{format}`"
      puts "| #{quoted_format.ljust(15)} | #{sample}"
    end
  end

  desc "Output the samples table in CSV format"
  task csv: %w[samples.csv] do |t|
    puts File.read('samples.csv')
  end

  desc "Output the samples table in JSON formt"
  task json: %w[samples.csv] do |t|
    require 'json'  # `gem install json`
    puts JSON.pretty_generate(SAMPLES)
  end

  desc "Output the samples table in YAML format"
  task yaml: %w[samples.csv] do |t|
    require 'yaml'  # `gem install yaml`
    puts SAMPLES.transform_keys(&:to_s).to_yaml
  end
end
