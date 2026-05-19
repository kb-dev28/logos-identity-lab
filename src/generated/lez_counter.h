/* Auto-generated C header for lez_counter FFI. DO NOT EDIT. */
#ifndef LEZ_COUNTER_FFI_H
#define LEZ_COUNTER_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

/* initialize instruction */
char* lez_counter_initialize(const char* args_json);

/* increment instruction */
char* lez_counter_increment(const char* args_json);

void lez_counter_free_string(char* s);
char* lez_counter_version(void);

#ifdef __cplusplus
}
#endif

#endif /* LEZ_COUNTER_FFI_H */
