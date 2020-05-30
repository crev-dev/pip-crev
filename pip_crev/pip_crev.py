"""pip-crev: Signed code reviews for Python packages."""
import argparse
import sys

from . import commands


class _Parser(argparse.ArgumentParser):
    def error(self, message):
        """Print help message on error."""
        sys.stderr.write("error: %s\n" % message)
        self.print_help()
        sys.exit(2)


def parse_command_line_arguments():
    """Parse command line arguments and execute relevant routine."""
    parser = _Parser(prog="pip-crev")
    commands.setup_parsers(parser)

    # Display help message if no further subcommand provided.
    def _print_help(*args, parser=parser, **kwargs):
        parser.print_help()

    parser.set_defaults(method=_print_help)

    arguments = parser.parse_args()
    arguments.method(**vars(arguments))
