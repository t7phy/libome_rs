#include <ome/ome.h>
#include <ome/integration_engine_gsl.h>
#include <ome/mellin.h>

using namespace ome;

struct ome_integration_result {
    int status;
    double result;
    double abs_error;
};

static int status_to_int(integration_status s) {
    switch(s) {
        case integration_status::success:                  return 0;
        case integration_status::max_iterations_reached:   return 1;
        case integration_status::rounding_error_detected:  return 2;
        case integration_status::singularity_detected:     return 3;
        case integration_status::divergence_detected:      return 4;
        case integration_status::precision_not_reached:    return 5;
        case integration_status::domain_error:             return 6;
        case integration_status::bad_tolerance:            return 7;
        case integration_status::other_error:              return 8;
        default:                                           return 8;
    }
}

// Mellin moment: binds (as, LM, NF) and integrates over x
#define MELLIN_MOMENT_FN(NAME) \
extern "C" ome_integration_result ome_mellin_moment_##NAME( \
    int n, double as, double lm, double nf, \
    double eps_abs, double eps_rel) \
{ \
    integration_engine_gsl engine; \
    auto mom = make_mellin_moment(NAME, engine, as, lm, nf); \
    auto [status, result, abs_error] = mom.integrate(n, eps_abs, eps_rel); \
    return { status_to_int(status), result, abs_error }; \
}

// Mellin convolution: binds (as, LM, NF), uses user-provided testfunc
#define MELLIN_CONVOLUTION_FN(NAME) \
extern "C" ome_integration_result ome_mellin_convolution_##NAME( \
    double x, double as, double lm, double nf, \
    double (*testfunc_ptr)(double), \
    double eps_abs, double eps_rel) \
{ \
    integration_engine_gsl engine; \
    std::function<double(double)> testfunc(testfunc_ptr); \
    auto conv = make_mellin_convolution(NAME, testfunc, engine, as, lm, nf); \
    auto [status, result, abs_error] = conv.integrate(x, eps_abs, eps_rel); \
    return { status_to_int(status), result, abs_error }; \
}

#define MELLIN_FNS(NAME) \
    MELLIN_MOMENT_FN(NAME) \
    MELLIN_CONVOLUTION_FN(NAME)

MELLIN_FNS(AqqQNSEven)
MELLIN_FNS(AqqQNSOdd)
MELLIN_FNS(AggQ)
MELLIN_FNS(AQqPS)
MELLIN_FNS(AQqPSs)
MELLIN_FNS(AqqQPS)
MELLIN_FNS(AqgQ)
MELLIN_FNS(AgqQ)
MELLIN_FNS(AQg)
MELLIN_FNS(polAqqQNSEven)
MELLIN_FNS(polAqqQNSOdd)
MELLIN_FNS(polAggQ)
MELLIN_FNS(polAQqPS)
MELLIN_FNS(polAQqPSs)
MELLIN_FNS(polAqqQPS)
MELLIN_FNS(polAqgQ)
MELLIN_FNS(polAgqQ)
MELLIN_FNS(polAQg)
