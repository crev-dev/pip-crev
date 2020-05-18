def setup_parser(parent_parser):
    parser = parent_parser.add_parser("switch", help="Change current ID")
    parser.set_defaults(method=_execute)
    parser.add_argument("id", help="New current ID")


def _execute(*arg1, **kwargs):
    print("very: ", arg1, kwargs)
