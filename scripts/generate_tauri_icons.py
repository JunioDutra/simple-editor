#!/usr/bin/env python3
"""Generate minimal icon assets required by tauri-build.

This avoids committing binary files directly while still providing
`src-tauri/icons/icon.png` and `src-tauri/icons/icon.ico` during CI.
"""

from __future__ import annotations

import base64
import struct
from pathlib import Path


def write_png(path: Path) -> None:
    # 1x1 transparent PNG
    png_b64 = (
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO7Z2ioAAAAASUVORK5CYII="
    )
    path.write_bytes(base64.b64decode(png_b64))


def write_ico(path: Path) -> None:
    # Minimal 1x1 32-bit ICO
    icon_dir = struct.pack("<HHH", 0, 1, 1)

    bitmap_info_header = struct.pack(
        "<IIIHHIIIIII",
        40,  # biSize
        1,  # biWidth
        2,  # biHeight (XOR + AND mask)
        1,  # biPlanes
        32,  # biBitCount
        0,  # biCompression
        4,  # biSizeImage
        0,  # biXPelsPerMeter
        0,  # biYPelsPerMeter
        0,  # biClrUsed
        0,  # biClrImportant
    )
    xor_bitmap = b"\x00\x00\x00\xff"  # BGRA
    and_mask = b"\x00\x00\x00\x00"  # row padded to 4 bytes
    image_data = bitmap_info_header + xor_bitmap + and_mask

    entry = struct.pack(
        "<BBBBHHII",
        1,  # width
        1,  # height
        0,  # color count
        0,  # reserved
        1,  # planes
        32,  # bit count
        len(image_data),  # bytes in res
        6 + 16,  # image offset
    )
    path.write_bytes(icon_dir + entry + image_data)


def main() -> None:
    icons_dir = Path("src-tauri/icons")
    icons_dir.mkdir(parents=True, exist_ok=True)

    write_png(icons_dir / "icon.png")
    write_ico(icons_dir / "icon.ico")

    print("Generated:", icons_dir / "icon.png", icons_dir / "icon.ico")


if __name__ == "__main__":
    main()
