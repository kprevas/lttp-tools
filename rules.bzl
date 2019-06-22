def patched_tilesheet_asm(name, rom, manifest_csv, sheet_num, pngs = []):
    native.genrule(
        name = name,
        srcs = [rom, manifest_csv] + pngs,
        tools = ["//:lttp_tilepatch"],
        outs = [name + ".asm"],
        cmd = "$(location //:lttp_tilepatch) $(location %s) patch -s %s -m $(location %s) -a $@" % (rom, sheet_num, manifest_csv),
    )
