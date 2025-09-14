#ifndef rust_bridge_h
#define rust_bridge_h

#include <stdbool.h>

// Function declarations for Rust FFI
extern void rust_main(void); // New main entry point that forks early
extern void rust_init(void); // Legacy init function
extern void set_accessibility_permission(bool has_permission);
extern bool get_accessibility_permission(void);

// Function that Swift implements for Rust to call
void swift_open_system_preferences(void);

#endif /* rust_bridge_h */