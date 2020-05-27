def setup_parser(parent_parser):
    parser = parent_parser.add_parser("new", help="Create new ID")
    parser.set_defaults(method=_execute)

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


def _execute(*arg, **kwargs):
    print("very: ", arg, kwargs)
