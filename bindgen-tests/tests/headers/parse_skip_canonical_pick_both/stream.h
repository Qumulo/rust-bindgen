struct stream_socket;
struct iostream;

__attribute__((overloadable))
struct iostream *iostream_cast(struct stream_socket *);
