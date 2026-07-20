abort("Expected Ruby 3.4+, but got #{RUBY_VERSION}.") if RUBY_VERSION < '3.4.0'

EDKY_SUBCOMMANDS = %w[list convert parse]

task default: %w[codegen]

desc "Generate .config/readmer/*.sh-session files"
task codegen: %w[.config/readmer/edky.sh-session] +
  EDKY_SUBCOMMANDS.map { ".config/readmer/edky-#{it}.sh-session" }.to_a

([nil] + EDKY_SUBCOMMANDS).each do |subcommand|
  command = subcommand ? "edky #{subcommand} --help" : "edky"
  filename = command.delete_suffix(' --help').gsub(' ', '-')
  desc "Generate .config/readmer/#{filename}.sh-session"
  file ".config/readmer/#{filename}.sh-session" do |t|
    File.open(t.name, 'w') do |f|
      f.puts "$ #{command}"
      f.puts `#{command} 2>&1`
    end
  end
end
