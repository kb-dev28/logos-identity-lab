/* Auto-generated C header for identity_lab FFI. DO NOT EDIT. */
#ifndef IDENTITY_LAB_FFI_H
#define IDENTITY_LAB_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

/* verify_age_proof instruction */
char* identity_lab_verify_age_proof(const char* args_json);

void identity_lab_free_string(char* s);
char* identity_lab_version(void);

#ifdef __cplusplus
}
#endif

#endif /* IDENTITY_LAB_FFI_H */
