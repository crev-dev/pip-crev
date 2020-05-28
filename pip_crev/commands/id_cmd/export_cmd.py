import crev


def setup_parser(parent_parser):
    parser = parent_parser.add_parser("export", help="Export ID")
    parser.set_defaults(method=_execute)
    parser.add_argument("id", help="ID to export", nargs="?")


def _execute(*arg, **kwargs):
    id_export = crev.id.export_id(kwargs.get("id"))
    print(id_export)
