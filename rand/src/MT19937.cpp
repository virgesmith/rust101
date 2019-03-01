#include <random>

extern "C" {

std::mt19937* mt19937_create(uint32_t seed)
{
  return new std::mt19937(seed);
}

void mt19937_destroy(std::mt19937* pimpl)
{
  delete pimpl;
}

uint32_t mt19937_next(std::mt19937* pimpl) 
{
  return (*pimpl)();
}

}

