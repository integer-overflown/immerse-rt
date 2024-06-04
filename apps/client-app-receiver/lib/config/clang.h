#ifndef IMMERSE_RT_CLANG_H
#define IMMERSE_RT_CLANG_H

#define IRT_HEADER_PRE                                                         \
    _Pragma("clang diagnostic push")                                           \
        _Pragma("clang diagnostic ignored \"-Wreturn-type-c-linkage\"")

#define IRT_HEADER_POST _Pragma("clang diagnostic pop")

#endif // IMMERSE_RT_CLANG_H
