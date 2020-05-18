def setup_parser(parent_parser):
    parser = parent_parser.add_parser("trust", help="Trust an ID")
    parser.set_defaults(method=_execute)
    parser.add_argument(
        "--no-commit",
        action="store_true",
        help="Don't auto-commit local Proof Repository",
    )
    parser.add_argument("--no-store", action="store_true", help="Don't store the proof")
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

    # TODO: change to singular
    parser.add_argument(
        "IDs", action="append", help="One or more (comma separated) public ID to trust"
    )


def _execute(*arg1, **kwargs):
    print("very: ", arg1, kwargs)
