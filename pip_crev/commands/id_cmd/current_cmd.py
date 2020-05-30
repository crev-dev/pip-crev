"""Current ID subcommand."""
import crev


def setup_parser(parent_parser):
    """CLI parser for `id current` subcommand."""
    parser = parent_parser.add_parser("current", help="Show your current ID")
    parser.set_defaults(method=_execute)


def _execute(*arg, **kwargs):
    crev.id.get_current()
