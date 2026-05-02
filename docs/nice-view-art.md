# Custom Art for nice!view Display

This documents the process for creating and integrating custom pixel art on the nice!view display for the Corne keyboard, using the custom shield at `config/boards/shields/nice_view_custom/`.

## Display Hardware

The nice!view uses a **Sharp LS011B7DH03** memory LCD:

- Resolution: **160x68 pixels**
- Color depth: **1-bit monochrome** (black and white only)
- The display is physically landscape (160 wide, 68 tall)

## Shield Overview

The custom shield replaces the stock `nice_view` shield. All source lives in `config/boards/shields/nice_view_custom/`.

Key files:

| File | Purpose |
|------|---------|
| `widgets/art.c` | Image data arrays and LVGL image descriptors |
| `widgets/peripheral_status.c` | Right-half display: custom art + battery/connection |
| `widgets/status.c` | Left-half display: battery, BLE profile, layer, WPM |
| `widgets/util.c` | Shared drawing helpers, canvas rotation, battery rendering |
| `widgets/util.h` | Constants (`CANVAS_SIZE = 68`), color macros, shared structs |
| `widgets/bolt.c` | Charging bolt icon (2-bit indexed) |
| `CMakeLists.txt` | Conditionally compiles `status.c` for central, `art.c` + `peripheral_status.c` for peripheral |
| `Kconfig.defconfig` | Display config: 1-bit color depth, DPI, memory pool, widget options |

## Split Display Layout

On a split Corne, each half builds different display code:

- **Left half (central):** Runs `status.c`. Shows three rotated canvas widgets stacked horizontally -- battery + BLE/USB + WPM graph, BLE profile selector (5 circles), and active layer name. No custom art.
- **Right half (peripheral):** Runs `peripheral_status.c` and `art.c`. Shows custom art on the left side of the display and a small status canvas (battery + connection icon) on the right.

The `CMakeLists.txt` controls this split:

```
if(NOT CONFIG_ZMK_SPLIT OR CONFIG_ZMK_SPLIT_ROLE_CENTRAL)
  zephyr_library_sources(widgets/status.c)
else()
  zephyr_library_sources(widgets/art.c)
  zephyr_library_sources(widgets/peripheral_status.c)
endif()
```

## Image Format

Images use the **LVGL indexed 1-bit** format (`LV_IMG_CF_INDEXED_1BIT`).

### Storage Dimensions

Images are stored as **140x68 pixels**. The full display is 160 wide, but the remaining 20 pixels on the right are used by the status canvas (battery/connection overlay drawn by `peripheral_status.c`).

### Data Layout

The raw byte array has two sections:

1. **Palette (8 bytes):** Two colors, 4 bytes each in BGRA order.
2. **Pixel data:** 1 bit per pixel, MSB first. Each row is padded to a whole byte boundary.

### Palette Bytes

Each palette entry is 4 bytes: `B, G, R, A`.

**Non-inverted (default):**

| Index | Meaning | Bytes |
|-------|---------|-------|
| 0 | White (background) | `0xff, 0xff, 0xff, 0xff` |
| 1 | Black (foreground) | `0x00, 0x00, 0x00, 0xff` |

**Inverted (`CONFIG_NICE_VIEW_WIDGET_INVERTED`):**

| Index | Meaning | Bytes |
|-------|---------|-------|
| 0 | Black (background) | `0x00, 0x00, 0x00, 0xff` |
| 1 | White (foreground) | `0xff, 0xff, 0xff, 0xff` |

The code uses `#if CONFIG_NICE_VIEW_WIDGET_INVERTED` to select which palette is compiled.

### Row Stride

Each row is `ceil(140 / 8) = 18 bytes`. Total pixel data: `18 * 68 = 1224 bytes`. With the 8-byte palette: `1224 + 8 = 1232 bytes` total (`data_size` in the descriptor).

### Image Descriptor

```c
const lv_img_dsc_t noface_open = {
    .header.cf = LV_IMG_CF_INDEXED_1BIT,
    .header.always_zero = 0,
    .header.reserved = 0,
    .header.w = 140,
    .header.h = 68,
    .data_size = 1232,
    .data = noface_open_map,
};
```

## Orientation and Rotation

### Design Orientation

Design your artwork in **portrait at 68x140 pixels** (68 wide, 140 tall). This is the natural "upright" orientation for the art as you want it to appear on the display.

### Storage Rotation

Before encoding to the C array, **rotate the image 90 degrees clockwise**. In Python with PIL:

```python
rotated = img.rotate(-90, expand=True)
```

This converts the 68x140 portrait design into the 140x68 landscape storage format. The nice!view display hardware and LVGL handle rendering this rotated data correctly on the screen.

### Why Rotate?

The nice!view display is physically 160x68 landscape, but the shield's canvas widgets use a rotation transform (`rotate_canvas` in `util.c` applies a 90-degree rotation via `lv_canvas_transform`). The art images bypass the canvas system and are placed directly with `lv_img_create` / `lv_img_set_src`, so they need to be pre-rotated to match the display's native orientation.

## Creating a New Image

### Step 1: Design

Create a **68x140 pixel** black-and-white image in any editor (Aseprite, GIMP, Photoshop, etc.). Use pure black (`#000000`) and pure white (`#ffffff`) only. White is background, black is foreground.

### Step 2: Convert

Write or use a Python script to:

1. Load the 68x140 image
2. Rotate 90 degrees clockwise (`img.rotate(-90, expand=True)`)
3. Convert to 1-bit (`img.convert('1')`)
4. Write the palette bytes (8 bytes for non-inverted, 8 for inverted)
5. Pack pixels MSB-first, padding each row to byte boundary
6. Output as a C array

Example conversion logic:

```python
from PIL import Image

img = Image.open("my_art.png").convert("1")
rotated = img.rotate(-90, expand=True)

width, height = rotated.size
row_bytes = (width + 7) // 8

palette_normal = bytes([0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0xff])
palette_inverted = bytes([0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff])

def pack_pixels(image):
    pixels = list(image.getdata())
    data = bytearray()
    for y in range(height):
        for byte_idx in range(row_bytes):
            byte_val = 0
            for bit in range(8):
                x = byte_idx * 8 + bit
                if x < width:
                    pixel = pixels[y * width + x]
                    if pixel == 0:
                        byte_val |= (1 << (7 - bit))
            data.append(byte_val)
    return data

pixel_data = pack_pixels(rotated)
```

Note: in PIL's 1-bit mode, `0` is black and `255` is white. Since index 1 in the palette is black, a black pixel (`0`) maps to bit value `1`.

### Step 3: Format as C

Structure the output as a C byte array with both palette variants:

```c
const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST uint8_t my_image_map[] = {
#if CONFIG_NICE_VIEW_WIDGET_INVERTED
    0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff,
    /* pixel data here */
#else
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0xff,
    /* pixel data here (same pixel bytes for both) */
#endif
};
```

The pixel data bytes are identical for both inverted and non-inverted variants. Only the 8-byte palette prefix differs.

### Step 4: Add the Descriptor

```c
const lv_img_dsc_t my_image = {
    .header.cf = LV_IMG_CF_INDEXED_1BIT,
    .header.always_zero = 0,
    .header.reserved = 0,
    .header.w = 140,
    .header.h = 68,
    .data_size = 1232,
    .data = my_image_map,
};
```

### Important: Linkage

The image descriptor (`lv_img_dsc_t`) and the byte array must **not** be declared `static` in `art.c`. They need external linkage so that `peripheral_status.c` can reference them via `LV_IMG_DECLARE`.

## Animation

The existing code implements a blink animation with two frames (`noface_open` and `noface_blink`).

### Declaring External Images

In the file that uses the images (e.g., `peripheral_status.c`), declare them:

```c
LV_IMG_DECLARE(noface_open);
LV_IMG_DECLARE(noface_blink);
```

### Creating the Image Widget

```c
noface_art = lv_img_create(widget->obj);
lv_img_set_src(noface_art, &noface_open);
lv_obj_align(noface_art, LV_ALIGN_TOP_LEFT, 0, 0);
```

### Timer-Based Animation

Use `lv_timer_create` to schedule frame swaps:

```c
lv_timer_create(blink_cb, 4000, NULL);
```

The callback swaps the image source:

```c
static void blink_cb(lv_timer_t *timer) {
    if (noface_art) {
        lv_img_set_src(noface_art, &noface_blink);
        lv_timer_create(blink_end_cb, 150, NULL);
    }
}

static void blink_end_cb(lv_timer_t *timer) {
    if (noface_art) {
        lv_img_set_src(noface_art, &noface_open);
    }
    lv_timer_del(timer);
}
```

This creates a blink effect: show the default frame for 4 seconds, swap to the blink frame for 150ms, then swap back. The pattern extends to any number of frames.

### More Complex Animation

For multi-frame animation, store an array of image pointers and cycle through them:

```c
LV_IMG_DECLARE(frame_0);
LV_IMG_DECLARE(frame_1);
LV_IMG_DECLARE(frame_2);

static const lv_img_dsc_t *frames[] = {&frame_0, &frame_1, &frame_2};
static int current_frame = 0;

static void animation_cb(lv_timer_t *timer) {
    current_frame = (current_frame + 1) % 3;
    lv_img_set_src(art_widget, frames[current_frame]);
}

lv_timer_create(animation_cb, 200, NULL);
```

## Build Configuration

### build.yaml

The build file at `keyboard/build.yaml` references `nice_view_custom` instead of the stock `nice_view`:

```yaml
include:
  - board: nice_nano_v2
    shield: corne_left nice_view_adapter nice_view_custom
  - board: nice_nano_v2
    shield: corne_right nice_view_adapter nice_view_custom
```

Both halves use the same shield name. The `CMakeLists.txt` inside the shield uses `CONFIG_ZMK_SPLIT_ROLE_CENTRAL` to compile different source files for each half.

### Kconfig

The shield's `Kconfig.defconfig` sets:

- `LV_Z_BITS_PER_PIXEL = 1`
- `LV_COLOR_DEPTH_1` (1-bit color)
- `LV_Z_MEM_POOL_SIZE = 4096` for custom screens
- `NICE_VIEW_WIDGET_STATUS` selects `LV_USE_IMG` and `LV_USE_CANVAS`
- `NICE_VIEW_WIDGET_INVERTED` is an optional bool to flip palette colors

### Display Overlay

The device tree overlay (`nice_view_custom.overlay`) defines the Sharp display:

```
nice_view: ls0xx@0 {
    compatible = "sharp,ls0xx";
    spi-max-frequency = <1000000>;
    width = <160>;
    height = <68>;
};
```

## Data Size Reference

| Parameter | Value |
|-----------|-------|
| Display resolution | 160x68 |
| Art image dimensions | 140x68 (stored), 68x140 (designed) |
| Bits per pixel | 1 |
| Bytes per row | 18 (ceil(140/8)) |
| Pixel data size | 1224 bytes (18 * 68) |
| Palette size | 8 bytes (2 colors * 4 bytes BGRA) |
| Total data_size | 1232 bytes |
| Color format | LV_IMG_CF_INDEXED_1BIT |
