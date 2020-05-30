"""Import ID subcommand."""
import crev


def setup_parser(parent_parser):
    """CLI parser for `id import` subcommand."""
    parser = parent_parser.add_parser("import", help="Import ID")
    parser.set_defaults(method=_execute)


def _execute(*arg, **kwargs):
    crev.id.import_id()
