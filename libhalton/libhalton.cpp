#include "halton_sampler.h"
#include "halton_enum.h"

extern "C" {
auto Halton_sampler_create_faure() -> Halton_sampler* {
    auto hs = new Halton_sampler();
    hs->init_faure();
    return hs;
}

auto Halton_sampler_get_num_dimensions(Halton_sampler* hs) -> unsigned {
    return hs->get_num_dimensions();
}

auto Halton_sampler_sample(Halton_sampler* hs, unsigned dimension, unsigned index) -> float {
    return hs->sample(dimension, index);
}

auto Halton_sampler_destroy(Halton_sampler* hs) {
    delete hs;
}

}