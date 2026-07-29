# See: https://docs.ruby-lang.org/en/4.0/Gem/Specification.html

require 'distrib/ruby/gemspec'

Distrib::Ruby::Gemspec.build!(__FILE__) do |gemspec|
  gemspec.summary     = "Edky for Ruby"
  gemspec.description = "Convert Ed25519 public keys between various encoding formats."
  gemspec.homepage    = "https://edky.dev"
  gemspec.metadata    = {
    :source_code_uri  => "https://github.com/artob/edky",
    :bug_tracker_uri  => "https://github.com/artob/edky/issues",
    :changelog_uri    => "https://github.com/artob/edky/blob/master/CHANGES.md",
  }.transform_keys(&:to_s)
end
