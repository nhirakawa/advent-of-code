from __future__ import print_function

from pathlib import Path

NUMERALS_TO_STRING = {
    1: 'one',
    2: 'two',
    3: 'three',
    4: 'four',
    5: 'five',
    6: 'six',
    7: 'seven',
    8: 'eight',
    9: 'nine',
    10: 'ten',
    11: 'eleven',
    12: 'twelve',
    13: 'thirteen',
    14: 'fourteen',
    15: 'fifteen',
    16: 'sixteen',
    17: 'seventeen',
    18: 'eighteen',
    19: 'nineteen',
    20: 'twenty',
    21: 'twenty_one',
    22: 'twenty_two',
    23: 'twenty_three',
    24: 'twenty_four',
    25: 'twenty_five',
}

def main():
    for year in [2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024]:
        src_path = f'src/year_{year}'

        print(f'Creating directory {src_path} (if not exists)')
        Path(src_path).mkdir(parents=True, exist_ok=True)

        for day in range(1, 26):
            day_string = NUMERALS_TO_STRING[day]
            day_path = f'{src_path}/day_{day_string}.rs'

            if not Path(day_path).exists():
                print(f'Creating file {day_path}')
                with open(day_path, 'w') as f:
                    f.write('use anyhow::bail;')
                    f.write('\n\n')
                    f.write('pub fn part_one(_input: &str) -> anyhow::Result<String> {')
                    f.write('\n')
                    f.write('\tbail!("Not implemented");')
                    f.write('\n')
                    f.write('}')

                    if day != 25:
                        f.write('\n')
                        f.write('pub fn part_two(_input: &str) -> anyhow::Result<String> {')
                        f.write('\n')
                        f.write('\tbail!("Not implemented");')
                        f.write('\n')
                        f.write('}')

if __name__ == '__main__':
    main()
