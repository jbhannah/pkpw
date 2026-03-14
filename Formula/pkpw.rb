class Pkpw < Formula
  desc "What if correct horse battery staple, but Pokémon"
  homepage "https://pkpw.jbhannah.net"
  url "https://github.com/jbhannah/pkpw/archive/refs/tags/v1.4.0.tar.gz"
  sha256 "d6db7646e8e6d393552681090c2447b77f3c04805843ae5c4c7e2cf2c764d776"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system bin / "pkpw", "--version"
  end
end
