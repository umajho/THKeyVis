#ifndef rust_bridge_h
#define rust_bridge_h

#include <stdbool.h>

// Function pointer type for permission monitoring callback
typedef void (*permission_monitoring_callback_t)(void);

// Function declarations for Rust FFI
extern void rust_main_with_callback(permission_monitoring_callback_t callback); // Main entry point with permission callback
extern void rust_main(void);                                                    // Main entry point (calls rust_main_with_callback with NULL)
extern void rust_init(void);                                                    // Legacy init function
extern void set_accessibility_permission(bool has_permission);
extern bool get_accessibility_permission(void);

// Function that Swift implements for Rust to call
void swift_open_system_preferences(void);
void swift_start_permission_monitoring(void); // Swift permission monitoring function

#endif /* rust_bridge_h */