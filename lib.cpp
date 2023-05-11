#include "include/lib.h"

#include "rusty_lib/lib.h"

std::string Fetcher::fetch() const {
  auto reqwester = new_requester();
  const auto result = reqwester->fetch();
  return std::string(result);
}