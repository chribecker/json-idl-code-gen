"""Implementation for the comgen_source rule."""

def _com_gen_source_impl(ctx):
    output_dir = ctx.actions.declare_directory(ctx.label.name)
    ctx.actions.run(
        inputs = [ctx.file.src],
        outputs = [output_dir],
        arguments = [
            "--input", ctx.file.src.path,
            "--output", output_dir.path,
            #"--namespace", ctx.attr.namespace,
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
        "namespace": attr.string(doc = "namespace filter to use for generated code", default = ""),
        "_tool": attr.label(
            default = Label("//:comgensource"),
            executable = True,
            cfg = "exec",
            doc = "The comgen_source Rust binary",
        ),
        "deps": attr.label_list(),
    },
)

def _com_gen_filter_impl(ctx):
    output_dir = ctx.actions.declare_directory(ctx.label.name)
    ctx.actions.run(
        inputs = [ctx.file.comgen],
        outputs = [output_dir],
        arguments = [
            "--input", ctx.file.comgen.path,
            "--output", output_dir.path,
            "--filter", ctx.attr.filter,
        ],
        executable = ctx.executable._tool,
        mnemonic = "ComgenFilter",
        progress_message = "Filter files from %s: %s" % (ctx.file.comgen.path, ctx.attr.filter),
    )
    return DefaultInfo(files = depset([output_dir]))

comgen_filter = rule(
    implementation = _com_gen_filter_impl,
    attrs = {
        "comgen": attr.label(allow_single_file = True, doc = "Reference to a comgen_source target"),
        "filter": attr.string(doc = "namespace filter to use for generated code", default = ""),
        "_tool": attr.label(
            default = Label("//:comgenfilter"),
            executable = True,
            cfg = "exec",
            doc = "The comgen_filter Rust binary",
        ),
    },
)

def _com_gen_debug_impl(ctx):
    output_dir = ctx.actions.declare_file(ctx.label.name+".txt")
    ctx.actions.run(
        inputs = [ctx.file.comgen],
        outputs = [output_dir],
        arguments = [
            "--name", ctx.label.name,
            "--input", ctx.file.comgen.path,
            "--output", output_dir.path,
        ],
        executable = ctx.executable._tool,
        mnemonic = "ComgenDebug",
        progress_message = "Debug print files from %s" % (ctx.file.comgen.path),
    )
    return DefaultInfo(files = depset([output_dir]))

comgen_debug = rule(
    implementation = _com_gen_debug_impl,
    attrs = {
        "comgen": attr.label(allow_single_file = True, doc = "Reference to a comgen_source target"),
        "_tool": attr.label(
            default = Label("//:comgendebug"),
            executable = True,
            cfg = "exec",
            doc = "The comgen_debug Rust binary",
        ),
    },
)


