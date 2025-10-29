"""Implementation for the comgen_source rule."""

def _com_gen_source_impl(ctx):

    # Declare a directory to stage the input files
    template_input_dir = ctx.actions.declare_directory(ctx.label.name + "_input_dir")
    output_dir = ctx.actions.declare_directory(ctx.label.name)
    args = [
        "--input", ctx.file.src.path,
        "--output", output_dir.path,
    ]

    if getattr(ctx.attr, "namespace", None):
        args.extend(["--namespace", ctx.attr.namespace])


    # Copy all srcs into the declared directory
    ctx.actions.run_shell(
        inputs = ctx.files.templates,
        outputs = [template_input_dir],
        command = """
        mkdir -p {input_dir}
        for f in {srcs}; do
            cp "$f" {input_dir}/
        done
        """.format(
            input_dir = template_input_dir.path,
            srcs = " ".join([f.path for f in ctx.files.templates]),
        ),
        mnemonic = "StageInputDir",
    )

    if getattr(ctx.attr, "templates", None):
        args.extend(["--templates", template_input_dir.path])


    ctx.actions.run(
        inputs = [ctx.file.src, template_input_dir],
        outputs = [output_dir],
        arguments = args,
        executable = ctx.executable._tool,
        mnemonic = "ComgenSource",
        progress_message = "Generating code from IDL: %s" % ctx.file.src.path,
    )
    return DefaultInfo(files = depset([output_dir]))

comgen_source = rule(
    implementation = _com_gen_source_impl,
    attrs = {
        "src": attr.label(allow_single_file = True, doc = "IDL file (JSON or YAML)"),
        "namespace": attr.string(doc = "namespace filter to use for generated code", default = ""),
        "templates": attr.label_list(allow_files = True, doc = "Optional templates directory for code generation"),
        "_tool": attr.label(
            default = Label("//:comgen"),
            executable = True,
            cfg = "exec",
            doc = "The comgen_source Rust binary",
        ),
        "deps": attr.label_list(),
    },
    output_to_genfiles = True,
)

def _com_gen_file_impl(ctx):
    output_file = ctx.actions.declare_file(ctx.attr.output if ctx.attr.output else ctx.label.name + ".txt")
    ctx.actions.run(
        inputs = [ctx.file.comgen],
        outputs = [output_file],
        arguments = [
            "--input", ctx.file.comgen.path,
            "--output", output_file.path,
            "--type", ctx.attr.type,
        ],
        executable = ctx.executable._tool,
        mnemonic = "ComgenFileExtract",
        progress_message = "Filter files from %s: %s" % (ctx.file.comgen.path, ctx.attr.type),
    )
    return DefaultInfo(files = depset([output_file]))

comgen_file = rule(
    implementation = _com_gen_file_impl,
    attrs = {
        "comgen": attr.label(allow_single_file = True, doc = "Reference to a comgen_source target"),
        "type": attr.string(doc = "type filter to use for generated code", default = ""),
        "output": attr.string(doc = "output file for generated code", default = ""),
        "_tool": attr.label(
            default = Label("//:comgenfile"),
            executable = True,
            cfg = "exec",
            doc = "The comgen_file Rust binary",
        ),
    },
    output_to_genfiles = True,
)
