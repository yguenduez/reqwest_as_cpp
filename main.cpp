#include "rusty_lib/lib.h"

#include <iostream>
#include <string>

int main() {
  // Opaque type
  auto reqwester = new_requester();
  const auto result = reqwester->fetch();
  std::cout << std::string{result} << std::endl;

  return 0;
}