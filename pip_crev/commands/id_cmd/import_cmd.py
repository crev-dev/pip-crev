def setup_parser(parent_parser):
    parser = parent_parser.add_parser("import", help="Import ID")
    parser.set_defaults(method=_execute)


def _execute(*arg, **kwargs):
    print("very: ", arg, kwargs)
