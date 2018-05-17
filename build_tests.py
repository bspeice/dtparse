#import dateutil.parser._timelex.split as time_split
from dateutil.parser import _timelex

# The TEST_STRINGS list should be the only thing that actually needs changing
TEST_STRINGS = [
    '2018.5.15',
    'May 5, 2018',
    'Mar. 5, 2018',
]

S4 = ' ' * 4
S8 = ' ' * 8
S12 = ' ' * 12

def test_string_to_rust(time_string):
    split_array = _timelex.split(time_string)

    def translate_token(token):
        if token[0].isalpha():
            return 'Token::Alpha("{}".to_owned())'.format(token)
        elif token[0].isnumeric():
            return 'Token::Numeric("{}".to_owned())'.format(token)
        elif len(token) == 1:
            return 'Token::Separator("{}".to_owned())'.format(token)
        else:
            raise Exception("Invalid token during parsing of dateutil "
                            "split: {}".format(token))

    return [translate_token(t) for t in split_array]

def main():
    header = '''use super::Token;
use super::tokenize;

#[test]
fn test_python_compat() {\n'''

    tests = []

    for test_string in TEST_STRINGS:
        token_string = '\n'.join(['{}{},'.format(S12, s)
                                  for s in test_string_to_rust(test_string)])
        tests.append('    assert_eq!(\n{}tokenize("{}"),\n{}vec![\n{}\n{}]\n{});'
                     .format(S8, test_string, S8, token_string, S8, S4))

    body = '\n'.join(tests)

    footer = '\n}\n'

    with open('src/test_python_compat.rs', 'w') as handle:
        handle.write(header)
        handle.write(body)
        handle.write(footer)

if __name__ == '__main__':
    main()