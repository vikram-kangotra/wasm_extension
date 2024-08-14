#include <stdlib.h>
#include "extension.h"

__attribute__((weak, export_name("canonical_abi_realloc")))
void *canonical_abi_realloc(
void *ptr,
size_t orig_size,
size_t org_align,
size_t new_size
) {
  void *ret = realloc(ptr, new_size);
  if (!ret)
  abort();
  return ret;
}

__attribute__((weak, export_name("canonical_abi_free")))
void canonical_abi_free(
void *ptr,
size_t size,
size_t align
) {
  free(ptr);
}
#include <string.h>

void extension_string_set(extension_string_t *ret, const char *s) {
  ret->ptr = (char*) s;
  ret->len = strlen(s);
}

void extension_string_dup(extension_string_t *ret, const char *s) {
  ret->len = strlen(s);
  ret->ptr = canonical_abi_realloc(NULL, 0, 1, ret->len);
  memcpy(ret->ptr, s, ret->len);
}

void extension_string_free(extension_string_t *ret) {
  canonical_abi_free(ret->ptr, ret->len, 1);
  ret->ptr = NULL;
  ret->len = 0;
}
void extension_description_free(extension_description_t *ptr) {
  extension_string_free(&ptr->title);
  extension_string_free(&ptr->version);
  extension_string_free(&ptr->author);
  extension_string_free(&ptr->shortdesc);
  extension_string_free(&ptr->description);
}

__attribute__((aligned(4)))
static uint8_t RET_AREA[40];
__attribute__((export_name("allocate-memory")))
int32_t __wasm_export_extension_allocate_memory(int32_t arg) {
  uint32_t ret = extension_allocate_memory((uint32_t) (arg));
  return (int32_t) (ret);
}
__attribute__((export_name("descriptor")))
int32_t __wasm_export_extension_descriptor(void) {
  extension_description_t ret;
  extension_descriptor(&ret);
  int32_t ptr = (int32_t) &RET_AREA;
  *((int32_t*)(ptr + 4)) = (int32_t) ((ret).title).len;
  *((int32_t*)(ptr + 0)) = (int32_t) ((ret).title).ptr;
  *((int32_t*)(ptr + 12)) = (int32_t) ((ret).version).len;
  *((int32_t*)(ptr + 8)) = (int32_t) ((ret).version).ptr;
  *((int32_t*)(ptr + 20)) = (int32_t) ((ret).author).len;
  *((int32_t*)(ptr + 16)) = (int32_t) ((ret).author).ptr;
  *((int32_t*)(ptr + 28)) = (int32_t) ((ret).shortdesc).len;
  *((int32_t*)(ptr + 24)) = (int32_t) ((ret).shortdesc).ptr;
  *((int32_t*)(ptr + 36)) = (int32_t) ((ret).description).len;
  *((int32_t*)(ptr + 32)) = (int32_t) ((ret).description).ptr;
  return ptr;
}
__attribute__((export_name("activate")))
void __wasm_export_extension_activate(void) {
  extension_activate();
}
__attribute__((export_name("deactivate")))
void __wasm_export_extension_deactivate(void) {
  extension_deactivate();
}
