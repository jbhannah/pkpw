class Pkpw < Formula
  desc "What if correct horse battery staple, but Pokémon."
  homepage "https://pkpw.jbhannah.net"
  url "https://github.com/jbhannah/pkpw/archive/refs/tags/v1.3.1.tar.gz"
  sha256 "771b7f3d73c4f304fd6e09b725d6984068c67ff7c7cf9f4cc030d0f79988dc9d"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system `brew --prefix pkpw`.chomp + "/bin/pkpw", "--version"
  end
end
