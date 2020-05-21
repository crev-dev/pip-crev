from . import id_cmd


def setup_parsers(parent_parser):
    subparsers = parent_parser.add_subparsers(
        title="subcommands", metavar="", dest="subparser_name"
    )
    id_cmd.setup_parser(subparsers)
