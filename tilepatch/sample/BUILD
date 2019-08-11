load("//:rules.bzl", "patched_link_sprites_asm", "patched_tilesheet_asm")

patched_tilesheet_asm(
    name = "tilegfx_bgd",
    manifest_csv = "manifest.csv",
    pngs = [
        "sample1.png",
        "sample2.png",
    ],
    rom = "dummy.sfc",
    sheet_num = 1,
)

patched_tilesheet_asm(
    name = "tilegfx_hud",
    manifest_csv = "manifest_2bpp.csv",
    pngs = [
        "sample2.png",
    ],
    rom = "dummy.sfc",
    sheet_num = 113,
)

patched_tilesheet_asm(
    name = "tilegfx_uncompressed_3bpp",
    manifest_csv = "manifest.csv",
    pngs = [
        "sample1.png",
        "sample2.png",
    ],
    rom = "dummy.sfc",
    sheet_num = 117,
)

patched_tilesheet_asm(
    name = "tilegfx_compressed_3bpp",
    manifest_csv = "manifest.csv",
    pngs = [
        "sample1.png",
        "sample2.png",
    ],
    rom = "dummy.sfc",
    sheet_num = 200,
)

patched_tilesheet_asm(
    name = "tilegfx_fonts",
    manifest_csv = "manifest_2bpp.csv",
    pngs = [
        "sample2.png",
    ],
    rom = "dummy.sfc",
    sheet_num = 219,
)

patched_link_sprites_asm(
    name = "tilegfx_link",
    manifest_csv = "manifest.csv",
    pngs = [
        "sample1.png",
        "sample2.png",
    ],
    rom = "dummy.sfc",
)
