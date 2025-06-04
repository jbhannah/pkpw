class Pkpw < Formula
  desc "What if correct horse battery staple, but Pokémon."
  homepage "https://pkpw.jbhannah.net"
  url "https://github.com/jbhannah/pkpw/archive/refs/tags/v1.3.2.tar.gz"
  sha256 "15d58b892bb5e7cef96871b9f8d6619662183f6fa0b35f38a6eea270f3b4d0a6"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system `brew --prefix pkpw`.chomp + "/bin/pkpw", "--version"
  end
end
