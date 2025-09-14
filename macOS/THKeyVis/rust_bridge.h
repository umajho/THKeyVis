#ifndef rust_bridge_h
#define rust_bridge_h

#include <stdbool.h>

// Function declarations for Rust FFI
extern void rust_init(void);
extern void set_accessibility_permission(bool has_permission);
extern bool get_accessibility_permission(void);

#endif /* rust_bridge_h */