#pragma once

// Second replacement candidate for `dup_target`. Bindgen's
// `ctx.replace()` is first-wins (subsequent entries warn and are
// dropped) — verifying that this stays first-wins, and that the
// behavior is the same with the lazy-parse flag on as with it off.
/// <div rustbindgen replaces="dup_target"></div>
struct dup_target_from_b {
    int field_b1;
    int field_b2;
};
