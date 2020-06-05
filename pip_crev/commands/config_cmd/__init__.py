"""Config subcommand."""
from . import edit_cmd


def setup_parser(parent_parser):
    """CLI parser for `config` subcommand."""
    parser = parent_parser.add_parser("config", help="Manage local crev configuration")

    # Display help message if no further subcommand provided.
    def _print_help(*args, parser=parser, **kwargs):
        parser.print_help()

    parser.set_defaults(method=_print_help)
    subparser = parser.add_subparsers(title="subcommands", metavar="")

    subcommands = [edit_cmd]
    for subcommand in subcommands:
        subcommand.setup_parser(subparser)
