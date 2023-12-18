#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct User {
  uint64_t id;
} User;

struct User get_user(void);
