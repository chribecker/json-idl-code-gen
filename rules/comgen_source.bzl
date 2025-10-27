"""Implementation for the comgen_source rule."""

def _com_gen_source_impl(ctx):
    output_dir = ctx.actions.declare_directory(ctx.label.name)
    ctx.actions.run(
        inputs = [ctx.file.src],
        outputs = [output_dir],
        arguments = [
            "--input", ctx.file.src.path,
            "--output", output_dir.path,
            "--namespace", ctx.attr.namespace,
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
    output_to_genfiles = True,
)

def _com_gen_filter_impl(ctx):
    output_dir = ctx.actions.declare_directory(ctx.label.name)
    ctx.actions.run(
        inputs = [ctx.file.comgen],
        outputs = [output_dir],
        arguments = [
            "--input", ctx.file.comgen.path,
            "--output", output_dir.path,
            "--group", ctx.attr.group,
        ],
        executable = ctx.executable._tool,
        mnemonic = "ComgenFilter",
        progress_message = "Filter files from %s: %s" % (ctx.file.comgen.path, ctx.attr.group),
    )
    return DefaultInfo(files = depset([output_dir]))

comgen_filter = rule(
    implementation = _com_gen_filter_impl,
    attrs = {
        "comgen": attr.label(allow_single_file = True, doc = "Reference to a comgen_source target"),
        "group": attr.string(doc = "group filter to use for generated code", default = ""),
        "_tool": attr.label(
            default = Label("//:comgenfilter"),
            executable = True,
            cfg = "exec",
            doc = "The comgen_filter Rust binary",
        ),
    },
    output_to_genfiles = True,
)

def _com_gen_debug_impl(ctx):
    output_dir = ctx.actions.declare_file(ctx.label.name+".txt")
    args =[]
    args.extend(["--output", output_dir.path])
    for src in ctx.files.srcs:
        args.extend(["--input", src.path])
    ctx.actions.run(
        inputs = ctx.files.srcs,
        outputs = [output_dir],
        arguments = args,
        executable = ctx.executable._tool,
        mnemonic = "FileDebug",
        progress_message = "Debug print files from %s" % (ctx.label.name),
    )
    return DefaultInfo(files = depset([output_dir]))

comgen_debug = rule(
    implementation = _com_gen_debug_impl,
    attrs = {
        "srcs": attr.label_list( doc = "Reference to a comgen_source target"),
        "_tool": attr.label(
            default = Label("//:comgendebug"),
            executable = True,
            cfg = "exec",
            doc = "The comgen_debug Rust binary",
        ),
    },
)

def _cc_comgen_library_impl(ctx):
    output = ctx.actions.declare_file(ctx.label.name+".txt")
    return DefaultInfo(files = depset([output]))

cc_comgen_library = rule(
    implementation = _cc_comgen_library_impl,
    attrs = {
        "comgen": attr.label(allow_single_file = True, doc = "Reference to a comgen_source target"),
    },
)

def _rust_comgen_library_impl(ctx):
    output = ctx.actions.declare_file(ctx.label.name+".txt")
    return DefaultInfo(files = depset([output]))

rust_comgen_library = rule(
    implementation = _rust_comgen_library_impl,
    attrs = {
        "comgen": attr.label(allow_single_file = True, doc = "Reference to a comgen_source target"),
    },
)

def _cc_skeleton_library_impl(ctx):
    output = ctx.actions.declare_file(ctx.label.name+".txt")
    return DefaultInfo(files = depset([output]))

cc_skeleton_library = rule(
    implementation = _cc_skeleton_library_impl,
    attrs = {
        "comgen": attr.label(allow_single_file = True, doc = "Reference to a comgen_source target"),
    },
)

