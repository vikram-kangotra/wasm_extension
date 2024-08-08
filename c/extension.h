#ifndef __BINDINGS_EXTENSION_H
#define __BINDINGS_EXTENSION_H
#include <stddef.h>
#ifdef __cplusplus
extern "C"
{
  #endif
  
  #include <stdint.h>
  #include <stdbool.h>
  
  typedef struct {
    char *ptr;
    size_t len;
  } extension_string_t;
  
  void extension_string_set(extension_string_t *ret, const char *s);
  void extension_string_dup(extension_string_t *ret, const char *s);
  void extension_string_free(extension_string_t *ret);
  typedef struct {
    extension_string_t title;
    extension_string_t version;
    extension_string_t author;
    extension_string_t shortdesc;
    extension_string_t description;
  } extension_description_t;
  void extension_description_free(extension_description_t *ptr);
  void extension_descriptor(extension_description_t *ret0);
  void extension_activate(void);
  void extension_deactivate(void);
  #ifdef __cplusplus
}
#endif
#endif
