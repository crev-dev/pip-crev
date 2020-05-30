"""ID subcommand."""
from . import (
    current_cmd,
    distrust_cmd,
    export_cmd,
    import_cmd,
    new_cmd,
    query_cmd,
    switch_cmd,
    trust_cmd,
    untrust_cmd,
)


def setup_parser(parent_parser):
    """CLI parser for `id` subcommand."""
    parser = parent_parser.add_parser("id", help="Manage all user IDs")

    # Display help message if no further subcommand provided.
    def _print_help(*args, parser=parser, **kwargs):
        parser.print_help()

    parser.set_defaults(method=_print_help)
    subparser = parser.add_subparsers(title="subcommands", metavar="")

    subcommands = [
        current_cmd,
        distrust_cmd,
        export_cmd,
        import_cmd,
        new_cmd,
        query_cmd,
        switch_cmd,
        trust_cmd,
        untrust_cmd,
    ]
    for subcommand in subcommands:
        subcommand.setup_parser(subparser)
