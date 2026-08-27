#include <zephyr/kernel.h>
#include <zmk/event_manager.h>
#include <zmk/behavior.h>
#include <zmk/events/position_state_changed.h>
#include <zmk/events/keycode_state_changed.h>

#include "caps_word_watcher.h"

ZMK_EVENT_IMPL(zmk_caps_word_state_changed);

static bool last_active = false;

static void check_and_notify(void) {
    const struct device *dev = zmk_behavior_get_binding("caps_word");
    if (dev == NULL) {
        return;
    }

    bool active = *(bool *)dev->data;
    if (active == last_active) {
        return;
    }

    last_active = active;
    raise_zmk_caps_word_state_changed((struct zmk_caps_word_state_changed){.active = active});
}

static int on_position_state_changed(const zmk_event_t *eh) {
    check_and_notify();
    return ZMK_EV_EVENT_BUBBLE;
}

static int on_keycode_state_changed(const zmk_event_t *eh) {
    check_and_notify();
    return ZMK_EV_EVENT_BUBBLE;
}

ZMK_LISTENER(cw_watcher_pos, on_position_state_changed);
ZMK_SUBSCRIPTION(cw_watcher_pos, zmk_position_state_changed);

ZMK_LISTENER(cw_watcher_kc, on_keycode_state_changed);
ZMK_SUBSCRIPTION(cw_watcher_kc, zmk_keycode_state_changed);
