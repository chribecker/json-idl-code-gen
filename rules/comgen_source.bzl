"""Implementation for the comgen_source rule."""

def _com_gen_source_impl(ctx):
    output_dir = ctx.actions.declare_directory(ctx.attr.out_dir)
    ctx.actions.run(
        inputs = [ctx.file.src],
        outputs = [output_dir],
        arguments = [
            "--input", ctx.file.src.path,
            "--output", output_dir.path,
        ],
        executable = ctx.executable._tool,
        mnemonic = "ComgenSource",
        progress_message = "Generating code from IDL: %s" % ctx.file.src.path,
    )
    return DefaultInfo(files = depset([output_dir]))

comgen_source = rule(
    implementation = _com_gen_source_impl,
    attrs = {
        "src": attr.label(allow_single_file = True, doc = "IDL file (JSON or YAML)"),
        "out_dir": attr.string(doc = "Output directory for generated code"),
        "_tool": attr.label(
            default = Label("//:comgen"),
            executable = True,
            cfg = "exec",
            doc = "The comgen Rust binary",
        ),
        "deps": attr.label_list(),
    },
)
