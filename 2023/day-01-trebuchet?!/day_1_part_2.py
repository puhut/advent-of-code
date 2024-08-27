import argparse
import re

WORD_TO_DIGIT = {
    'one': '1',
    'two': '2',
    'three': '3',
    'four': '4',
    'five': '5',
    'six': '6',
    'seven': '7',
    'eight': '8',
    'nine': '9'
}

def get_digits(line):
    positions = [
        (id, digit) 
        for word, digit in WORD_TO_DIGIT.items()
        for id in range(len(line))
        if line.startswith(word, id)
    ]

    positions.extend((id, character) for id, character in enumerate(line) if character.isdigit())

    positions.sort(key=lambda tup: tup[0])

    first, last = positions[0][1], positions[-1][1]
    return int(first + last)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--input_file_name', dest='input_file_name', type=str, help='input puzzle\'s input file name')
    args = parser.parse_args()

    print(args.input_file_name)

    with open(args.input_file_name) as input_data:
        sum = 0

        for line in input_data:
            sum += get_digits(line)
        
        print(f'total is {sum}')