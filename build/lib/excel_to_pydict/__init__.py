import sys, json
from ._native import lib, ffi
from io import StringIO


class Capturing(list):
    def __enter__(self):
        self._stdout = sys.stdout
        sys.stdout = self._stringio = StringIO()
        return self
    def __exit__(self, *args):
        self.extend(self._stringio.getvalue().splitlines())
        del self._stringio    # free up some memory
        sys.stdout = self._stdout


def convert(f):
    with Capturing() as output:
        lib.print_xlsx_file(f.encode("utf-8"))
    return json.loads(output[0])