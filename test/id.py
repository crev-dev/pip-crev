def setup_parser(parent_parser):
    parser = parent_parser.add_parser("id", help="Manage all user IDs")

    # Display ID subcommand help message if no further subcommand provided.
    def _print_help(*args, parser=parser, **kwargs):
        parser.print_help()

    parser.set_defaults(method=_print_help)
    subparser = parser.add_subparsers(title="subcommands", metavar="")

    _setup_parser_current(subparser)
    _setup_parser_distrust(subparser)
    _setup_parser_export(subparser)
    _setup_parser_import(subparser)
    _setup_parser_new(subparser)
    _setup_parser_query(subparser)
    _setup_parser_switch(subparser)
    _setup_parser_trust(subparser)
    _setup_parser_untrust(subparser)


def _setup_parser_current(parent_parser):
    parser = parent_parser.add_parser("current", help="Show your current ID")
    parser.set_defaults(method=_execute_subcommand_current)


def _execute_subcommand_current(*arg1, **kwargs):
    print("very: ", arg1, kwargs)


def _setup_parser_distrust(parent_parser):
    parser = parent_parser.add_parser("distrust", help="Distrust an ID")
    parser.set_defaults(method=_execute_subcommand_distrust)
    parser.add_argument(
        "--no-commit",
        action="store_true",
        help="Don't auto-commit local Proof Repository",
    )
    parser.add_argument(
        "--no-store", action="store_true", help="Don't store the proof",
    )
    parser.add_argument(
        "--print-signed",
        action="store_true",
        help="Print signed proof content to stdout",
    )
    parser.add_argument(
        "--print-unsigned",
        action="store_true",
        help="Print unsigned proof content to stdout",
    )
    parser.add_argument(
        "IDs",
        action="append",
        help="One or more (comma separated) public ID to distrust",
    )


def _execute_subcommand_distrust(*arg1, **kwargs):
    print("very: ", arg1, kwargs)


def _setup_parser_export(parent_parser):
    parser = parent_parser.add_parser("export", help="Export ID")
    parser.set_defaults(method=_execute_subcommand_export)


def _execute_subcommand_export(*arg1, **kwargs):
    print("very: ", arg1, kwargs)


def _setup_parser_import(parent_parser):
    parser = parent_parser.add_parser("import", help="Import ID")
    parser.set_defaults(method=_execute_subcommand_import)


def _execute_subcommand_import(*arg1, **kwargs):
    print("very: ", arg1, kwargs)


def _setup_parser_new(parent_parser):
    parser = parent_parser.add_parser("new", help="Create new ID")
    parser.set_defaults(method=_execute_subcommand_new)

    parser.add_argument(
        "--https-push",
        action="store_true",
        help="Setup `https` instead of recommended `ssh`-based push url",
    )
    parser.add_argument(
        "--url",
        action="store",
        help="URL of a git repository to be associated with the new ID",
    )
    parser.add_argument(
        "--github-username", action="store", help="Github username (instead of --url)",
    )


def _execute_subcommand_new(*arg1, **kwargs):
    print("very: ", arg1, kwargs)


def _setup_parser_query(parent_parser):
    parser = parent_parser.add_parser("query", help="Query IDs")
    parser.set_defaults(method=_execute_subcommand_query)


def _execute_subcommand_query(*arg1, **kwargs):
    print("very: ", arg1, kwargs)


def _setup_parser_switch(parent_parser):
    parser = parent_parser.add_parser("switch", help="Change current ID")
    parser.set_defaults(method=_execute_subcommand_switch)


def _execute_subcommand_switch(*arg1, **kwargs):
    print("very: ", arg1, kwargs)


def _setup_parser_trust(parent_parser):
    parser = parent_parser.add_parser("trust", help="Trust an ID")
    parser.set_defaults(method=_execute_subcommand_trust)


def _execute_subcommand_trust(*arg1, **kwargs):
    print("very: ", arg1, kwargs)


def _setup_parser_untrust(parent_parser):
    parser = parent_parser.add_parser("untrust", help="Untrust an ID")
    parser.set_defaults(method=_execute_subcommand_untrust)


def _execute_subcommand_untrust(*arg1, **kwargs):
    print("very: ", arg1, kwargs)
