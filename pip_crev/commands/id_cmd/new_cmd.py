"""New ID subcommand."""
import sys

import crev


def setup_parser(parent_parser):
    """CLI parser for `id new` subcommand."""
    parser = parent_parser.add_parser("new", help="Create new ID")

    def execute(*arg, **kwargs):
        _execute(*arg, parser=parser, **kwargs)

    parser.set_defaults(method=execute)

    parser.add_argument(
        "--https-push",
        action="store_true",
        help="Setup `https` instead of recommended `ssh`-based push url",
    )
    parser.add_argument(
        "--url",
        action="store",
        help="URL of a git repository to be associated with the new ID",
    )
    parser.add_argument(
        "--github-username", action="store", help="Github username (instead of --url)"
    )


def _execute(*arg, parser=None, **kwargs):
    try:
        crev.id.new(
            kwargs.get("url"), kwargs.get("github_username"), kwargs.get("https_push")
        )
    except OSError as error:
        print(error, "\n", file=sys.stderr)
        parser.print_help()
