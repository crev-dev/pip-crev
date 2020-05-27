def setup_parser(parent_parser):
    parser = parent_parser.add_parser("query", help="Query IDs")

    # Display help message if no further subcommand provided.
    def _print_help(*args, parser=parser, **kwargs):
        parser.print_help()

    parser.set_defaults(method=_print_help)

    subparser = parser.add_subparsers(title="subcommands", metavar="")
    _setup_parser_all(subparser)
    _setup_parser_current(subparser)
    _setup_parser_own(subparser)
    _setup_parser_trusted(subparser)


def _setup_parser_all(parent_parser):
    parser = parent_parser.add_parser("all", help="Show all known IDs")
    parser.set_defaults(method=_execute_all)
    parser.add_argument(
        "--depth",
        default=10,
        help="Maximum allowed distance from the root identity when traversing trust graph [default: 10]",
    )
    parser.add_argument("--for-id")  # TODO: what is this?
    parser.add_argument(
        "--high-cost",
        default=0,
        help="Cost of traversing trust graph edge of high trust level [default: 0]",
    )
    parser.add_argument(
        "--medium-cost",
        default=1,
        help="Cost of traversing trust graph edge of medium trust level [default: 1]",
    )
    parser.add_argument(
        "--low-cost",
        default=5,
        help="Cost of traversing trust graph edge of low trust level [default: 5]",
    )


def _execute_all(*arg, **kwargs):
    print("very: ", arg, kwargs)


def _setup_parser_current(parent_parser):
    parser = parent_parser.add_parser("current", help="Show current ID")
    parser.set_defaults(method=_execute_current)
    parser.add_argument(
        "--depth",
        default=10,
        help="Maximum allowed distance from the root identity when traversing trust graph [default: 10]",
    )
    parser.add_argument(
        "--high-cost",
        default=0,
        help="Cost of traversing trust graph edge of high trust level [default: 0]",
    )
    parser.add_argument(
        "--medium-cost",
        default=1,
        help="Cost of traversing trust graph edge of medium trust level [default: 1]",
    )
    parser.add_argument(
        "--low-cost",
        default=5,
        help="Cost of traversing trust graph edge of low trust level [default: 5]",
    )


def _execute_current(*arg, **kwargs):
    print("very: ", arg, kwargs)


def _setup_parser_own(parent_parser):
    parser = parent_parser.add_parser("own", help="Show own IDs")
    parser.set_defaults(method=_execute_own)
    parser.add_argument(
        "--depth",
        default=10,
        help="Maximum allowed distance from the root identity when traversing trust graph [default: 10]",
    )
    parser.add_argument(
        "--high-cost",
        default=0,
        help="Cost of traversing trust graph edge of high trust level [default: 0]",
    )
    parser.add_argument(
        "--medium-cost",
        default=1,
        help="Cost of traversing trust graph edge of medium trust level [default: 1]",
    )
    parser.add_argument(
        "--low-cost",
        default=5,
        help="Cost of traversing trust graph edge of low trust level [default: 5]",
    )


def _execute_own(*arg, **kwargs):
    print("very: ", arg, kwargs)


def _setup_parser_trusted(parent_parser):
    parser = parent_parser.add_parser("trusted", help="Show all trusted IDs")
    parser.set_defaults(method=_execute_trusted)
    parser.add_argument(
        "--depth",
        default=10,
        help="Maximum allowed distance from the root identity when traversing trust graph [default: 10]",
    )
    parser.add_argument("--for-id")  # TODO: what is this?
    parser.add_argument(
        "--high-cost",
        default=0,
        help="Cost of traversing trust graph edge of high trust level [default: 0]",
    )
    parser.add_argument(
        "--medium-cost",
        default=1,
        help="Cost of traversing trust graph edge of medium trust level [default: 1]",
    )
    parser.add_argument(
        "--low-cost",
        default=5,
        help="Cost of traversing trust graph edge of low trust level [default: 5]",
    )
    parser.add_argument(
        "--trust", default="low", help="Minimum trust level required [default: low]"
    )


def _execute_trusted(*arg, **kwargs):
    print("very: ", arg, kwargs)
