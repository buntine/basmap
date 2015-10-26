require 'formula'

class Basmap < Formula
  homepage 'https://github.com/buntine/basmap'
  url 'https://github.com/buntine/basmap/archive/v0.6.1-alpha.tar.gz'
  sha256 '9e30e97c08cdba0a2e6de182583ac5aa918815c94f5d962321615aef194319f7'
  version '0.6.1-alpha'

  head 'https://github.com/buntine/basmap.git'

  depends_on 'rust'

  def install
    system 'cargo build --release'
    bin.install 'target/release/basmap'
  end
end
