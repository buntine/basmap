require 'formula'

class Basmap < Formula
  homepage 'https://github.com/buntine/basmap'
  url 'https://github.com/buntine/basmap/archive/v0.6.2.tar.gz'
  sha256 'e58b0a6b8a9b937ab5b170eba24168e42889febeb088040f0b9ee774fecf7b51'
  version '0.6.2'

  head 'https://github.com/buntine/basmap.git'

  depends_on 'rust'

  def install
    system 'cargo build --release'
    bin.install 'target/release/basmap'
  end
end
