Gem::Specification.new do |gem|
  gem.version            = File.read('VERSION').chomp
  gem.date               = File.mtime('VERSION').strftime('%Y-%m-%d')

  gem.name               = "edky"
  gem.homepage           = "https://edky.dev"
  gem.license            = "Unlicense"
  gem.summary            = "Edky for Ruby"
  gem.description        = "Convert Ed25519 public keys between various encoding formats."
  gem.metadata           = {
    'bug_tracker_uri'   => "https://github.com/artob/edky/issues",
    'changelog_uri'     => "https://github.com/artob/edky/blob/master/CHANGES.md",
    'documentation_uri' => "https://rubydoc.info/gems/edky",
    'homepage_uri'      => "https://edky.dev",
    'source_code_uri'   => "https://github.com/artob/edky",
  }

  gem.author             = "Arto Bendiken"
  gem.email              = "arto@bendiken.net"

  gem.platform           = Gem::Platform::RUBY
  gem.files              = %w(AUTHORS CHANGES.md README.md UNLICENSE VERSION) + Dir.glob('lib/**/*.rb')
  gem.bindir             = %q(bin)
  gem.executables        = %w()

  gem.required_ruby_version = '>= 4.0'
  gem.add_development_dependency 'rspec', '~> 3.13'
  gem.add_development_dependency 'yard' , '~> 0.9'
end
