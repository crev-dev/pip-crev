def setup_parser(parent_parser):
    parser = parent_parser.add_parser("import", help="Import ID")
    parser.set_defaults(method=_execute)


def _execute(*arg1, **kwargs):
    print("very: ", arg1, kwargs)
