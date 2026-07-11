# This is free and unencumbered software released into the public domain.

module Edky; end
module Edky::VERSION; end

module Edky::VERSION
  FILE = File.expand_path('../../../VERSION', __FILE__)
  STRING = File.read(FILE).chomp.freeze
  MAJOR, MINOR, PATCH, EXTRA = STRING.split('.').map(&:freeze)
end # Edky::VERSION
