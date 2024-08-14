#include "extension.h"
#include <string.h>

extern void msg_dbg(const char* message, int length);

void vlc_msg_dbg(const char* message) {
    msg_dbg(message, strlen(message));
}

void extension_descriptor(extension_description_t* ptr) {
    extension_string_set(&ptr->title, "test");
    extension_string_set(&ptr->version, "0.0.1");
    extension_string_set(&ptr->author, "VideoLAN");
    extension_string_set(&ptr->shortdesc, "Test example");
    extension_string_set(&ptr->description, "Test description");
}

void extension_activate(void) {
    vlc_msg_dbg("Hello, World!");
}

void extension_deactivate(void) {
}
