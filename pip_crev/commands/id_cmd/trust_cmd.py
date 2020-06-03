"""Trust ID subcommand."""
import crev


def setup_parser(parent_parser):
    """CLI parser for `id trust` subcommand."""
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


def _execute(*arg, **kwargs):
    print(kwargs)
    crev.id.wrap_create_trust_proof(
        ids=kwargs["IDs"],
        trust_or_distrust="Trust",
        no_commit=kwargs["no_commit"],
        print_unsigned=kwargs["print_unsigned"],
        print_signed=kwargs["print_signed"],
        no_store=kwargs["no_store"],
    )
