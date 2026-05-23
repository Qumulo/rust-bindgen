struct tcp_socket;
struct iostream;

__attribute__((overloadable))
struct iostream *iostream_cast(struct tcp_socket *);
