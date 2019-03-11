#include "sobol.h"

extern "C" {

auto sobol_sample(unsigned long long index, const unsigned dimension, const unsigned scramble) -> float {
    return sobol::sample(index, dimension, scramble);
}

}