import argparse
import re

parser = argparse.ArgumentParser()
parser.add_argument('--input_file_name', dest='input_file_name', type=str, help='input puzzle\'s input file name')
args = parser.parse_args()

print(args.input_file_name)

with open(args.input_file_name) as input_data:
    sum = 0

    for line in input_data:
        result=re.findall(r'\d', line)
        sum += int(result[0] + result[-1])
    
    print(f'total is {sum}')