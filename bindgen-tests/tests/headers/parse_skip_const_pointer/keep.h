struct handle_value;

const struct handle_value* get_handle(void);
void take_handle(const struct handle_value* h);

struct holder {
    const struct handle_value* held;
};
