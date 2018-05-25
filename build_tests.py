#import dateutil.parser._timelex.split as time_split
from dateutil.parser import _timelex
from dateutil.parser import parse as duparse
import pytz

# The TEST_STRINGS list should be the only thing that actually needs changing
TEST_STRINGS = [
    '2018.5.15',
    'May 5, 2018',
    'Mar. 5, 2018',
    '19990101T23',
    '19990101T2359',
]

AUTOGEN_HEADER = '''
// WARNING
// This file was auto-generated using the `build_tests.py` script.
// Please do not edit it manually.

'''

S4 = ' ' * 4
S8 = ' ' * 8
S12 = ' ' * 12

def rust_tokenize(time_string):
    split_array = _timelex.split(time_string)
    return ['"{}".to_owned()'.format(token) for token in split_array]

def build_split_string_tests():
    header = '''use tokenize;

#[test]
fn test_python_compat() {\n'''

    tests = []

    for test_string in TEST_STRINGS:
        token_string = '\n'.join(['{}{},'.format(S12, s)
                                  for s in rust_tokenize(test_string)])
        tests.append('    assert_eq!(\n{}tokenize("{}"),\n{}vec![\n{}\n{}]\n{});'
                     .format(S8, test_string, S8, token_string, S8, S4))

    body = '\n'.join(tests)

    footer = '\n}\n'

    return header + body + footer

def test_parse(time_string):
    dt = duparse(time_string)
    # TODO: Don't make this dependent on New_York
    iso8601 = pytz.timezone('America/New_York').localize(dt).astimezone(pytz.utc)
    return 'assert_eq!(\n{}parse("{}".to_owned())\n{}.unwrap()\n{}.to_rfc3339_opts(SecondsFormat::Micros, false),\n{}"{}"\n{});'.format(
        S8, time_string, S12, S12, S8, iso8601, S4)

def build_parse_tests():
    header = '''use chrono::SecondsFormat;

use parse;

#[test]
fn test_python_compat() {\n'''

    asserts = ['    {}'.format(test_parse(a)) for a in TEST_STRINGS]
    body = '\n'.join(asserts)

    footer = '\n}\n'

    return header + body + footer

if __name__ == '__main__':
    split_string_test = build_split_string_tests()
    with open('src/tests/compat_split_string.rs', 'w+') as handle:
        handle.write(AUTOGEN_HEADER + split_string_test)

    parse_test = build_parse_tests()
    with open('src/tests/compat_parse.rs', 'w+') as handle:
        handle.write(AUTOGEN_HEADER + parse_test)