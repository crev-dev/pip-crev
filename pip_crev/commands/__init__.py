"""Command line interface (CLI) commands."""
from . import config_cmd, id_cmd


def setup_parsers(parent_parser):
    """Initialise CLI parsers for all subcommands."""
    subparsers = parent_parser.add_subparsers(
        title="subcommands", metavar="", dest="subparser_name"
    )
    id_cmd.setup_parser(subparsers)
    config_cmd.setup_parser(subparsers)
