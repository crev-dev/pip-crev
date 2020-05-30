"""Switch ID subcommand."""
import crev


def setup_parser(parent_parser):
    """CLI parser for `id switch` subcommand."""
    parser = parent_parser.add_parser("switch", help="Change current ID")
    parser.set_defaults(method=_execute)
    parser.add_argument("id", help="New current ID")


def _execute(*arg, **kwargs):
    crev.id.switch(kwargs["id"])
