#ifndef PRO_READER_H
#define PRO_READER_H

#if __cplusplus
extern "C"
{
#endif

    int read_int(const char *);
    float read_float(const char *);
    char *read_string(const char *);
    short read_bool(const char *);

#if __cplusplus
}
#endif

#endif