def setup_parser(parent_parser):
    parser = parent_parser.add_parser("export", help="Export ID")
    parser.set_defaults(method=_execute)
    parser.add_argument("id", help="ID to export")


def _execute(*arg, **kwargs):
    print("very: ", arg, kwargs)
