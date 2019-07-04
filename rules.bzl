def patched_tilesheet_asm(
        name,
        rom,
        manifest_csv,
        sheet_num,
        pngs = [],
        bank_table_addr = "",
        high_table_addr = "",
        low_table_addr = "",
        asm_module = "",
        asm_label = ""):
    asm_module_arg = "--asm_module %s" % asm_module if asm_module else ""
    asm_label_arg = "--asm_label %s" % asm_label if asm_label else ""
    native.genrule(
        name = name,
        srcs = [rom, manifest_csv] + pngs,
        tools = ["//tilepatch:lttp_tilepatch"],
        outs = [name + ".asm"],
        cmd = "$(location //tilepatch:lttp_tilepatch) $(location %s) %s %s %s patch -s %s -m $(location %s) -a $@ %s %s" %
              (
                  rom,
                  bank_table_addr,
                  high_table_addr,
                  low_table_addr,
                  sheet_num,
                  manifest_csv,
                  asm_module_arg,
                  asm_label_arg,
              ),
    )

def patched_link_sprites_asm(
        name,
        rom,
        manifest_csv,
        pngs = [],
        addr = "",
        asm_module = "",
        asm_label = ""):
    addr_arg = "--sheet_addr %s" % addr if addr else ""
    asm_module_arg = "--asm_module %s" % asm_module if asm_module else ""
    asm_label_arg = "--asm_label %s" % asm_label if asm_label else ""
    native.genrule(
        name = name,
        srcs = [rom, manifest_csv] + pngs,
        tools = ["//tilepatch:lttp_tilepatch"],
        outs = [name + ".asm"],
        cmd = "$(location //tilepatch:lttp_tilepatch) $(location %s) patch_link -m $(location %s) -a $@ %s %s %s" %
              (
                  rom,
                  manifest_csv,
                  addr_arg,
                  asm_module_arg,
                  asm_label_arg,
              ),
    )

def text_asm(
        name,
        src,
        asm_module = "",
        asm_label = "",
        asm_addr = ""):
    asm_module_arg = "--asm_module %s" % asm_module if asm_module else ""
    asm_label_arg = "--asm_label %s" % asm_label if asm_label else ""
    asm_addr_arg = "--asm_addr %s" % asm_addr if asm_addr else ""
    native.genrule(
        name = name,
        srcs = [src],
        tools = ["//textconvert:lttp_textconvert"],
        outs = [name + ".asm"],
        cmd = "$(location //textconvert:lttp_textconvert) $(location %s) $@ %s %s %s" %
                (
                    src,
                    asm_module_arg,
                    asm_label_arg,
                    asm_addr_arg,
                ),
    )