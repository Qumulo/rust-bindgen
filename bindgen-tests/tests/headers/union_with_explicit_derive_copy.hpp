// bindgen-flags: --no-derive-copy


/// <div rustbindgen derive="Copy" derive="Clone"></div>
struct myCopyStruct { int i; };

typedef struct myCopyStruct myTypedef;

union myUnion {
    struct myCopyStruct i;
    myTypedef t;
    int primitiveInt;
};
