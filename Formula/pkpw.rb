class Pkpw < Formula
  desc "What if correct horse battery staple, but Pokémon."
  homepage "https://pkpw.jbhannah.net"
  url "https://github.com/jbhannah/pkpw/archive/refs/tags/v1.3.2.tar.gz"
  sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system `brew --prefix pkpw`.chomp + "/bin/pkpw", "--version"
  end
end
