"""Config edit subcommand."""
import crev


def setup_parser(parent_parser):
    """CLI parser for `config edit` subcommand."""
    parser = parent_parser.add_parser("edit", help="Edit local crev config file")
    parser.set_defaults(method=_execute)


def _execute(*arg, **kwargs):
    crev.config.edit()
